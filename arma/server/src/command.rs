use crate::{actor, bank, garage, locker, organization, v_garage, v_locker};

pub(crate) fn dispatch(command: &str, args: Vec<String>) -> String {
    match command {
        "actor:init" => unary(args, actor::init_actor),
        "actor:disconnect" => unary(args, actor::disconnect_actor),
        "actor:disconnect_uid" => unary(args, actor::disconnect_actor_uid),
        "actor:get" => unary(args, actor::get_actor),
        "actor:delete" => unary(args, actor::delete_actor),
        "bank:init" => ternary(args, bank::init_bank),
        "bank:disconnect" => unary(args, bank::disconnect_bank),
        "garage:init" => unary(args, garage::init_garage),
        "garage:disconnect" => unary(args, garage::disconnect_garage),
        "garage:get" => unary(args, garage::get_garage),
        "garage:save" => unary(args, garage::save_garage),
        "garage:delete" => unary(args, garage::delete_garage),
        "locker:init" => unary(args, locker::init_locker),
        "locker:disconnect" => unary(args, locker::disconnect_locker),
        "locker:get" => unary(args, locker::get_locker),
        "locker:save" => unary(args, locker::save_locker),
        "locker:delete" => unary(args, locker::delete_locker),
        "organization:create_default" => ternary(args, organization::create_default),
        "organization:create_player" => ternary(args, organization::create_player),
        "organization:add_member" => binary(args, organization::add_member),
        "organization:get" => unary(args, organization::get_organization),
        "organization:get_by_member" => unary(args, organization::get_by_member),
        "organization:issue_payday" => quaternary(args, organization::issue_payday),
        "v_garage:init" => binary(args, v_garage::init_garage),
        "v_garage:disconnect" => unary(args, v_garage::disconnect_garage),
        "v_garage:get" => unary(args, v_garage::get_garage),
        "v_garage:save" => unary(args, v_garage::save_garage),
        "v_garage:delete" => unary(args, v_garage::delete_garage),
        "v_locker:init" => binary(args, v_locker::init_locker),
        "v_locker:disconnect" => unary(args, v_locker::disconnect_locker),
        "v_locker:get" => unary(args, v_locker::get_locker),
        "v_locker:save" => unary(args, v_locker::save_locker),
        "v_locker:delete" => unary(args, v_locker::delete_locker),
        _ => format!("Error: Unsupported transport route: {command}"),
    }
}

fn unary(args: Vec<String>, f: fn(String) -> String) -> String {
    let Ok([a]) = <[String; 1]>::try_from(args) else {
        return "Error: invalid argument count".to_string();
    };
    f(a)
}

fn binary(args: Vec<String>, f: fn(String, String) -> String) -> String {
    let Ok([a, b]) = <[String; 2]>::try_from(args) else {
        return "Error: invalid argument count".to_string();
    };
    f(a, b)
}

fn ternary(args: Vec<String>, f: fn(String, String, String) -> String) -> String {
    let Ok([a, b, c]) = <[String; 3]>::try_from(args) else {
        return "Error: invalid argument count".to_string();
    };
    f(a, b, c)
}

fn quaternary(args: Vec<String>, f: fn(String, String, String, String) -> String) -> String {
    let Ok([a, b, c, d]) = <[String; 4]>::try_from(args) else {
        return "Error: invalid argument count".to_string();
    };
    f(a, b, c, d)
}
