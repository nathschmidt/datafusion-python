// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::fmt::{Debug, Display};

use pyo3::PyErr;

pub fn py_type_err(e: impl Debug + Display) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("{e}"))
}

pub fn py_runtime_err(e: impl Debug + Display) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}"))
}

pub fn py_value_err(e: impl Debug + Display) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}"))
}
