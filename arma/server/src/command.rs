use crate::{
    actor, bank, garage, locker, medical, notification, organization, rearm, refuel, repair,
    v_garage, v_locker,
};

pub(crate) fn dispatch(command: &str, args: Vec<String>) -> String {
    match command {
        "actor:init" => unary(args, actor::init_actor),
        "actor:disconnect" => unary(args, actor::disconnect_actor),
        "actor:disconnect_uid" => unary(args, actor::disconnect_actor_uid),
        "actor:get" => unary(args, actor::get_actor),
        "actor:delete" => unary(args, actor::delete_actor),
        "bank:init" => ternary(args, bank::init_bank),
        "bank:get" => unary(args, bank::get_bank),
        "bank:deposit" => binary(args, bank::deposit_bank),
        "bank:withdraw" => binary(args, bank::withdraw_bank),
        "bank:transfer" => ternary(args, bank::transfer_bank),
        "bank:disconnect" => unary(args, bank::disconnect_bank),
        "refuel:quote" => ternary(args, refuel::quote),
        "refuel:complete" => quinary(args, refuel::refuel_complete),
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
        "medical:respawn" => binary(args, medical::medical_respawn),
        "medical:heal" => binary(args, medical::medical_heal),
        "notification:list" => unary(args, notification::list_notifications),
        "notification:unread" => unary(args, notification::unread_notifications),
        "notification:mark_read" => binary(args, notification::mark_read_notification),
        "notification:mark_all_read" => unary(args, notification::mark_all_read_notifications),
        "organization:create_default" => ternary(args, organization::create_default),
        "organization:create_player" => ternary(args, organization::create_player),
        "organization:disband" => binary(args, organization::disband),
        "organization:create_invite" => ternary(args, organization::create_invite),
        "organization:accept_invite" => binary(args, organization::accept_invite),
        "organization:decline_invite" => binary(args, organization::decline_invite),
        "organization:leave_member" => binary(args, organization::leave_member),
        "organization:kick_member" => ternary(args, organization::kick_member),
        "organization:add_member" => binary(args, organization::add_member),
        "organization:get" => unary(args, organization::get_organization),
        "organization:get_by_member" => unary(args, organization::get_by_member),
        "organization:issue_payday" => quaternary(args, organization::issue_payday),
        "rearm:quote" => binary(args, rearm::rearm_quote),
        "rearm:complete" => quaternary(args, rearm::rearm_complete),
        "repair:quote" => binary(args, repair::repair_quote),
        "repair:complete" => quaternary(args, repair::repair_complete),
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

fn quinary(args: Vec<String>, f: fn(String, String, String, String, String) -> String) -> String {
    let Ok([a, b, c, d, e]) = <[String; 5]>::try_from(args) else {
        return "Error: invalid argument count".to_string();
    };
    f(a, b, c, d, e)
}
