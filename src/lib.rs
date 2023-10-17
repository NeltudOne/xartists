#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// const DAY_IN_SECONDS: u64 = 3600 * 24;
// const DAY_IN_SECONDS: u64 = 10;
// const YEAR_IN_DAYS: u64 = 365;
// const TOTAL_PERCENTAGE: u64 = 10000;

const NFT_AMOUNT: u32 = 1; // Token has to be unique to be considered NFT
const EGLD_IN_WEI: u64 = 1_000_000_000_000_000_000u64;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::contract]
pub trait TroSwap
{
    #[init]
    fn init(&self, token_id: TokenIdentifier, token_price: BigUint, presale_goal_amount: BigUint) {
        self.token_id().set(token_id);
        self.token_price().set(token_price);
        self.presale_goal_amount().set(presale_goal_amount);

        self.pause().set(false); // live
    }

    #[payable("*")]
    #[endpoint(buyNft)]
    fn buy_nft(&self,
        #[payment_token] swap_token_id: TokenIdentifier,
        #[payment_amount] token_amount: BigUint,
        nft_token_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: BigUint,
    ) {
        self.require_activation();

        require!(
            self.swap_token_ids().contains(&swap_token_id),
            "invalid swap_token_id"
        );

        require!(
            self.nft_token_ids().contains(&nft_token_id),
            "invalid nft_token_id"
        );

        require!(
            self.nft_price(swap_token_id.clone(), nft_token_id.clone()).get() > BigUint::zero(),
            "can not buy nft"
        );

        require!(
            token_amount >= self.nft_price(swap_token_id.clone(), nft_token_id.clone()).get(),
            "can not buy less then nft_price"
        );

        require!(
            nft_amount == BigUint::from(NFT_AMOUNT),
            "Must tranfer one"
        );

        require!(
            nft_token_id.is_valid_esdt_identifier(),
            "invalid nft_token_id"
        );

        require!(
            nft_nonce > 0,
            "Only NFT can swap"
        );

        require!(
            self.blockchain().get_sc_balance(&nft_token_id, nft_nonce) > BigUint::zero(),
            "not exist the NFT"
        );

        self.send().direct(&self.blockchain().get_caller(), &nft_token_id, nft_nonce, &nft_amount, &[]);
    }

    #[payable("EGLD")]
    #[endpoint(buy)]
    fn buy(&self, #[payment_amount] payment_amount: BigUint) {
        self.require_activation();

        let buy_amount = BigUint::from(EGLD_IN_WEI) * &payment_amount / &self.token_price().get();

        require!(
            self.presale_bought_amount().get() + buy_amount.clone() <= self.presale_goal_amount().get(),
            "can not buy more than goal amount"
        );
        self.presale_bought_amount().update(|v| *v += &buy_amount);;

        self.send().direct(&caller, &self.token_id().get(), 0, &buy_amount, &[]);
    }

    // inline
    #[inline]
    fn require_activation(&self) {
        require!(
            self.pause().get() == false,
            "swap is not live"
        );
    }

    // owner
    #[only_owner]
    #[endpoint(withdrawFunds)]
    fn withdraw_funds(&self,
        opt_token_id: OptionalValue<TokenIdentifier>,
        opt_token_nonce: OptionalValue<u64>,
        opt_token_amount: OptionalValue<BigUint>
    ) {
        // if token_id is not given, set it to eGLD
        let token_id = match opt_token_id {
            OptionalValue::Some(v) => v,
            OptionalValue::None => TokenIdentifier::egld()
        };

        // if token_id is not given, set it to 0
        let token_nonce = match opt_token_nonce {
            OptionalValue::Some(v) => v,
            OptionalValue::None => 0,
        };

        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let token_amount = match opt_token_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_sc_balance(&token_id, 0)
        };

        self.send().direct(&self.blockchain().get_caller(), &token_id, token_nonce, &token_amount, &[]);
    }

    #[only_owner]
    #[endpoint(addNftPrice)]
    fn add_nft_price(
        &self, 
        swap_token_id: TokenIdentifier, 
        nft_token_id: TokenIdentifier,
        nft_price: BigUint,
    ) {
        if !self.swap_token_ids().contains(&swap_token_id) {
            self.swap_token_ids().insert(swap_token_id.clone());
        }

        if !self.nft_token_ids().contains(&nft_token_id) {
            self.nft_token_ids().insert(nft_token_id.clone());
        }

        self.nft_price(swap_token_id.clone(), nft_token_id.clone()).set(nft_price);
    }

    #[only_owner]
    #[endpoint(removeSwapTokenId)]
    fn remove_swap_token_id(&self, swap_token_id: TokenIdentifier) {
        self.swap_token_ids().remove(&swap_token_id);
    }

    #[only_owner]
    #[endpoint(clearSwapTokenIds)]
    fn clear_swap_token_ids(&self) {
        self.swap_token_ids().clear();
    }

    #[only_owner]
    #[endpoint(removeNftTokenId)]
    fn remove_nft_token_id(&self, nft_token_id: TokenIdentifier) {
        self.nft_token_ids().remove(&nft_token_id);
    }

    #[only_owner]
    #[endpoint(clearNftTokenIds)]
    fn clear_nft_token_ids(&self) {
        self.nft_token_ids().clear();
    }

    #[only_owner]
    #[endpoint(setTokenId)]
    fn set_token_id(&self, token_id: TokenIdentifier) {
        self.token_id().set(token_id);
    }

    #[only_owner]
    #[endpoint(setTokenPrice)]
    fn set_token_id(&self, token_price: BigUint) {
        self.token_price().set(token_price);
    }

    // storage
    #[view(getSwapTokenIds)]
    #[storage_mapper("swap_token_ids")]
    fn swap_token_ids(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[view(getNftTokenIds)]
    #[storage_mapper("nft_token_ids")]
    fn nft_token_ids(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[view(getNftPrice)]
    #[storage_mapper("nft_price")]
    fn nft_price(&self, swap_token_id: TokenIdentifier, nft_token_id: TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getPasued)]
    #[storage_mapper("pause")]
    fn pause(&self) -> SingleValueMapper<bool>;

    #[view(getTokenId)]
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTokenPrice)]
    #[storage_mapper("token_price")]
    fn token_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getPresaleGoalAmount)]
    #[storage_mapper("presale_goal_amount")]
    fn presale_goal_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getPresaleBoughtAmount)]
    #[storage_mapper("presale_bought_amount")]
    fn presale_bought_amount(&self) -> SingleValueMapper<BigUint>;

}
