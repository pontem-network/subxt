// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use subxt::ClientBuilder;

/// metadata for encoding and decoding
mod metadata;
use metadata::pontem_mod::api as pontem;

/// Implementation of the missing "traits"
const _: () = {
    use pontem::runtime_types::polkadot_parachain::primitives::Id;

    impl PartialEq for Id {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Id {}

    impl PartialOrd for Id {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Id {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("ws://127.0.0.1:9944")
        .build()
        .await?
        .to_runtime_api::<pontem::RuntimeApi<pontem::DefaultConfig>>();

    let block_number = 1;

    let block_hash = api
        .client
        .rpc()
        .block_hash(Some(block_number.into()))
        .await?;

    if let Some(hash) = block_hash {
        println!("Block hash for block number {}: {}", block_number, hash);
    } else {
        println!("Block number {} not found.", block_number);
    }

    Ok(())
}
