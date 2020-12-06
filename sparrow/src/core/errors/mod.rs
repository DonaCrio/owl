// Copyright [2020] [Donatien Criaud]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod egg_not_in_nest_error;
mod poisoned_queue_error;
mod sparrow_error;

pub use egg_not_in_nest_error::EggNotInNestError;
pub use poisoned_queue_error::{PoisonedInputQueueError, PoisonedOutputQueueError};
pub use sparrow_error::{Result, SparrowError};
