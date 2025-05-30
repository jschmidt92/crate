use tokio::spawn;
use tokio::time::Duration;
use forge_models::actor::Actor;

use crate::events::ForgeEvent;
use crate::messaging::MessagingSystem;

impl Actor {
    pub fn new(uid: String) -> Self {
        Self {
            uid,
            position: None,
            direction: None, 
            stance: None,
            email: None,
            phone: None,
            bank: 0.0,
            cash: 0.0,
            state: None
        }
    }

    pub fn get_position(&self) -> Option<(f32, f32, f32)> {
        self.position
    }

    pub fn get_direction(&self) -> Option<f32> {
        self.direction
    }

    pub fn get_stance(&self) -> Option<String> {
        self.stance.clone()
    }

    pub fn get_email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn get_phone(&self) -> Option<String> {
        self.phone.clone()
    }

    pub fn get_bank(&self) -> f64 {
        self.bank
    }

    pub fn get_cash(&self) -> f64 {
        self.cash
    }

    pub fn deposit_to_bank(&mut self, amount: f64) {
        if self.cash >= amount {
            self.cash -= amount;
            self.bank += amount;
            Ok(())
        } else {
            Err("Insufficient funds in wallet".to_string())
        }
    }

    pub fn withdraw_from_bank(&mut self, amount: f64) -> Result<(), String> {
        if self.bank >= amount {
            self.bank -= amount;
            self.cash += amount;
            Ok(())
        } else {
            Err("Insufficient funds in bank account".to_string())
        }
    }

    pub fn get_state(&self) -> Option<String> {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: String) {
        self.state = Some(state)
    }
}

pub fn register_actor_handlers(msq: &mut MessagingSystem) {
    msq.register_handler("ActorPositionResult", |event| async move {
        if let ForgeEvent::ActorPositionResult { uid, position } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorPosition", |event| async move {
        if let ForgeEvent::UpdateActorPosition { uid, new_position } = event {
            // TODO: implement
        }
    });
    msq.register_handler("ActorDirectionResult", |event| async move {
        if let ForgeEvent::ActorDirectionResult { uid, direction } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorDirection", |event| async move {
        if let ForgeEvent::UpdateActorDirection { uid, new_direction } = event {
            // TODO: implement
        }
    });
    msq.register_handler("ActorStanceResult", |event| async move {
        if let ForgeEvent::ActorStanceResult { uid, stance } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorStance", |event| async move {
        if let ForgeEvent::UpdateActorStance { uid, new_stance } = event {
            // TODO: implement
        }
    });
    msq.register_handler("ActorEmailResult", |event| async move {
        if let ForgeEvent::ActorEmailResult { uid, email } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorEmail", |event| async move {
        if let ForgeEvent::UpdateActorEmail { uid, new_email } = event {
            // TODO: implement
        }
    });
    msq.register_handler("ActorPhoneResult", |event| async move {
        if let ForgeEvent::ActorPhoneResult { uid, phone } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorPhone", |event| async move {
        if let ForgeEvent::UpdateActorPhone { uid, new_phone } = event {
            // TODO: implement
        }
    });
    msq.register_handler("ActorBankBalanceResult", |event| async move {
        if let ForgeEvent::ActorBankBalanceResult { uid, balance } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorBank", |event| async move {
        if let ForgeEvent::UpdateActorBank { uid, new_balance } = event {
            // TODO: implement
        }
    });
    msq.register_handler("ActorCashBalanceResult", |event| async move {
        if let ForgeEvent::ActorCashBalanceResult { uid, balance } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorCash", |event| async move {
        if let ForgeEvent::UpdateActorCash { uid, new_balance } = event {
            // TODO: implement
        }
    });
    msq.register_handler("ActorStateBalanceResult", |event| async move {
        if let ForgeEvent::ActorStateBalanceResult { uid, state } = event {
            // TODO: implement
        }
    });
    msq.register_handler("UpdateActorState", |event| async move {
        if let ForgeEvent::UpdateActorState { uid, new_state } = event {
            // TODO: implement
        }
    })
}