use std::vec;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserTwo {
    two_vec: Vector<String>,
}
//实现默认初始化方法
impl Default for UserTwo {
    fn default() -> Self {
        Self {
            two_vec: Vector::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl UserTwo{
    //添加元素
    pub fn two_add(&mut self, item:String) {
        let sig_act_id = env::signer_account_id().to_string();
        let cur_act_id = env::signer_account_id().to_string();
        let tmp = format!("signer_account_id:{};_current_account_id:{};_mess:{}", sig_act_id, cur_act_id, item);
        self.two_vec.push(&tmp);
    }
    //判断元素是否存在
    pub fn two_exists(&self, item:String) ->bool {
        if self.two_vec.len() == 0 {
            return false;
        }
        for obj in self.two_vec.iter() {
            if obj == item {
                return true;
            }
        }
        true
    }
    //遍历所有元素
    pub fn two_list(&self) ->Vec<String> {
        let mut tmp = Vec::new();
        for obj in self.two_vec.iter() {
            tmp.push(obj);
        }
        tmp
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn two_add() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut two: UserTwo = UserTwo::default();
        two.two_add("11111111".to_string());
        two.two_add("22222222".to_string());
        two.two_add("33333333".to_string());
        println!("添加成功！");
    }

    #[test]
    fn two_exists() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut two: UserTwo = UserTwo::default();
        two.two_add("11111111".to_string());
        two.two_add("22222222".to_string());
        two.two_add("33333333".to_string());
        if two.two_exists("11111111".to_string()) {
            println!("存在111111111");
        } else {
            println!("不存在111111111");
        }
    }

    #[test]
    fn two_list() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut two: UserTwo = UserTwo::default();
        two.two_add("11111111".to_string());
        two.two_add("22222222".to_string());
        two.two_add("33333333".to_string());

        for item in two.two_list() {
            println!("item: {}", &item);
        }
    }

}
