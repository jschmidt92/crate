use crate::{command, log};
use arma_rs::Group;
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

const CHUNK_PREFIX: &str = "FORGE_TRANSPORT_CHUNK:";
const RESPONSE_CHUNK_SIZE: usize = 12_000;

static REQUESTS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static RESPONSES: LazyLock<Mutex<HashMap<String, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn group() -> Group {
    Group::new()
        .command("invoke", invoke)
        .command("invoke_stored", invoke_stored)
        .command("request:append", request_append)
        .command("response:get", response_get)
        .command("response:clear", response_clear)
}

fn invoke(command: String, args_json: String) -> String {
    let args = match serde_json::from_str::<Vec<String>>(&args_json) {
        Ok(args) => args,
        Err(error) => {
            log::error(format_args!("invalid transport arguments: {error}"));
            return format!("Error: invalid transport arguments: {error}");
        }
    };

    chunk_response(command::dispatch(&command, args))
}

fn invoke_stored(command: String, transfer_id: String) -> String {
    let args_json = {
        let Ok(mut requests) = REQUESTS.lock() else {
            return "Error: transport request store unavailable".to_string();
        };
        requests.remove(&transfer_id)
    };

    match args_json {
        Some(args_json) => invoke(command, args_json),
        None => "Error: transport request not found".to_string(),
    }
}

fn request_append(transfer_id: String, chunk: String) -> String {
    if transfer_id.trim().is_empty() {
        return "Error: invalid transport request id".to_string();
    }

    let Ok(mut requests) = REQUESTS.lock() else {
        return "Error: transport request store unavailable".to_string();
    };

    requests
        .entry(transfer_id)
        .and_modify(|value| value.push_str(&chunk))
        .or_insert(chunk);

    "OK".to_string()
}

fn response_get(transfer_id: String, index: String) -> String {
    let index = match index.parse::<usize>() {
        Ok(index) => index,
        Err(_) => return "Error: invalid transport response index".to_string(),
    };

    let Ok(responses) = RESPONSES.lock() else {
        return "Error: transport response store unavailable".to_string();
    };

    responses
        .get(&transfer_id)
        .and_then(|chunks| chunks.get(index))
        .cloned()
        .unwrap_or_else(|| "Error: transport response chunk not found".to_string())
}

fn response_clear(transfer_id: String) -> String {
    let Ok(mut responses) = RESPONSES.lock() else {
        return "Error: transport response store unavailable".to_string();
    };

    responses.remove(&transfer_id);
    "OK".to_string()
}

fn chunk_response(response: String) -> String {
    if response.len() <= RESPONSE_CHUNK_SIZE {
        return response;
    }

    let transfer_id = format!("res_{}", uuid::Uuid::new_v4());
    let chunks = split_chunks(&response, RESPONSE_CHUNK_SIZE);
    let envelope = ChunkEnvelope {
        transfer_id: transfer_id.clone(),
        chunk_count: chunks.len(),
    };

    let Ok(mut responses) = RESPONSES.lock() else {
        return "Error: transport response store unavailable".to_string();
    };
    responses.insert(transfer_id, chunks);

    match serde_json::to_string(&envelope) {
        Ok(json) => format!("{CHUNK_PREFIX}{json}"),
        Err(error) => format!("Error: failed to serialize transport envelope: {error}"),
    }
}

fn split_chunks(value: &str, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();

    for character in value.chars() {
        if current.len() + character.len_utf8() > chunk_size && !current.is_empty() {
            chunks.push(std::mem::take(&mut current));
        }
        current.push(character);
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ChunkEnvelope {
    transfer_id: String,
    chunk_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_chunks_preserves_utf8_boundaries() {
        let chunks = split_chunks("aaébb", 3);

        assert_eq!(
            chunks,
            vec!["aa".to_string(), "éb".to_string(), "b".to_string()]
        );
    }

    #[test]
    fn chunk_response_stores_large_response_for_follow_up_reads() {
        let response = "a".repeat(RESPONSE_CHUNK_SIZE + 10);

        let envelope_response = chunk_response(response.clone());
        assert!(envelope_response.starts_with(CHUNK_PREFIX));

        let envelope: serde_json::Value =
            serde_json::from_str(&envelope_response[CHUNK_PREFIX.len()..]).unwrap();
        let transfer_id = envelope["transferId"].as_str().unwrap().to_string();
        let chunk_count = envelope["chunkCount"].as_u64().unwrap() as usize;

        let mut assembled = String::new();
        for index in 0..chunk_count {
            assembled.push_str(&response_get(transfer_id.clone(), index.to_string()));
        }
        response_clear(transfer_id);

        assert_eq!(assembled, response);
    }
}
