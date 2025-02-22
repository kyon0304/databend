// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_datavalues::DataSchemaRef;

use crate::Expression;
use crate::PlanNode;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AggregatorPartialPlan {
    pub group_expr: Vec<Expression>,
    pub aggr_expr: Vec<Expression>,
    pub schema: DataSchemaRef,
    pub input: Arc<PlanNode>,
}

impl AggregatorPartialPlan {
    pub fn set_input(&mut self, node: &PlanNode) {
        self.input = Arc::new(node.clone());
    }

    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}
