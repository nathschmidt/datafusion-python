# Test Coverage Analysis for DataFusion Python

This document analyzes the current test coverage of the datafusion-python codebase and identifies areas that need improvement.

## Executive Summary

The datafusion-python project has approximately **192 test functions** across **20 test files** (~5,935 lines of test code). While core functionality is reasonably well-tested, there are significant gaps in several critical areas.

### Key Findings

| Area | Current Coverage | Priority |
|------|-----------------|----------|
| Math Functions | ~97% | Low (well covered) |
| String Functions | ~84% | Medium |
| Array/List Functions | ~100% | Low (well covered) |
| Hash Functions | ~100% | Low (well covered) |
| **Aggregation Functions** | **~29%** | **CRITICAL** |
| **Window Functions** | **~0%** | **CRITICAL** |
| User-Defined Functions | ~40% | High |
| Object Store | ~5% | High |
| Common Types | ~5% | Medium |
| Error Handling | ~5% | High |

---

## Critical Coverage Gaps

### 1. Aggregation Functions (CRITICAL)

**Location**: `python/datafusion/functions.py`
**Test File**: `python/tests/test_functions.py`

The basic SQL aggregation functions have almost no dedicated tests:

**NOT TESTED (22 functions):**
- **Basic Aggregates**: `avg()`, `count()`, `count_star()`, `max()`, `min()`, `sum()`
- **Statistical**: `corr()`, `covar()`, `covar_pop()`, `covar_samp()`, `median()`, `stddev()`, `stddev_pop()`, `stddev_samp()`, `var()`, `var_pop()`, `var_samp()`, `var_sample()`
- **Bitwise**: `bit_and()`, `bit_or()`, `bit_xor()`, `bool_and()`, `bool_or()`
- **Approximate**: `approx_distinct()`, `approx_median()`, `approx_percentile_cont()`, `approx_percentile_cont_with_weight()`
- **String**: `string_agg()`

**Recommendation**: Create `python/tests/test_aggregation_functions.py` with comprehensive tests for all aggregation functions.

### 2. Window Functions (CRITICAL)

**Location**: `python/datafusion/functions.py`
**Test File**: None dedicated

**ZERO COVERAGE for these functions:**
- `row_number()`
- `rank()`
- `dense_rank()`
- `lag()`
- `lead()`
- `first_value()`
- `last_value()`
- `nth_value()`
- `cume_dist()`
- `percent_rank()`
- `ntile()`

**Recommendation**: Create `python/tests/test_window_functions.py` with tests covering:
- Basic window function usage
- PARTITION BY clauses
- ORDER BY clauses
- Window frame specifications (ROWS, RANGE)
- Combinations of partition/order/frame

---

## High Priority Coverage Gaps

### 3. Object Store Module

**Location**: `python/datafusion/object_store.py`
**Test File**: `python/tests/test_store.py` (minimal)

| Object Store | Test Coverage |
|--------------|---------------|
| `LocalFileSystem` | NOT TESTED |
| `AmazonS3` | NOT TESTED |
| `GoogleCloud` | NOT TESTED |
| `MicrosoftAzure` | NOT TESTED |
| `Http` | Minimal (1 test) |

**Recommendation**: Add tests for:
- Object store instantiation with various configurations
- Registration with SessionContext
- Reading files from each store type
- Error handling for invalid credentials/endpoints

### 4. User-Defined Functions (UDF/UDAF/UDWF)

**Location**: `python/datafusion/udf.py`
**Test Files**: `test_udf.py`, `test_udaf.py`, `test_udwf.py`

#### Missing Coverage:

| Feature | ScalarUDF | UDAF | UDWF |
|---------|:---------:|:----:|:----:|
| Multi-argument functions | ❌ | ❌ | ✅ |
| Null array handling | ❌ | ❌ | ❌ |
| Empty array handling | ❌ | ❌ | ❌ |
| Volatility=Stable | ❌ | ❌ | ❌ |
| Volatility=Volatile | ❌ | ❌ | ❌ |
| Enum volatility values | ❌ | ❌ | ❌ |
| Multiple return types | ❌ | ❌ | ❌ |
| Name auto-generation | ❌ | ❌ | ⚠️ |
| Error propagation | ❌ | ❌ | ⚠️ |

**Legend**: ✅ Tested, ⚠️ Partially tested, ❌ Not tested

**Specific Gaps**:

**ScalarUDF (`test_udf.py`):**
- Only tests `pa.bool_()` return type - missing int, float, string, complex types
- No multi-argument UDF tests
- No volatility variations tested
- No error handling tests

**UDAF (`test_udaf.py`):**
- Only tests `pa.float64()` return type
- Known bug: code breaks on None (line 37 comment) - not tested
- No multi-state accumulator tests
- Limited merge operation testing

**UDWF (`test_udwf.py`):**
- Missing tests for `memoize()`, `is_causal()` methods
- Only 2 window frame types tested
- No tests for flag combinations

### 5. Error Handling

**Current State**: Only ~24 error handling tests across entire test suite

**Missing Error Cases**:
- Invalid SQL queries (malformed syntax)
- Schema mismatch errors
- Type coercion failures
- Resource exhaustion scenarios
- Invalid configuration options
- Invalid UDF return types
- Stream operation errors
- File not found scenarios
- Permission errors

**Recommendation**: Add error handling tests to each module's test file.

---

## Medium Priority Coverage Gaps

### 6. SessionContext Methods

**Location**: `python/datafusion/context.py`
**Test File**: `python/tests/test_context.py`

**Untested Methods**:
- `session_id()` - Returns unique session identifier
- `empty_table()` - Creates empty DataFrame
- `enable_url_table()` - Enables querying local files as tables
- `register_table_provider()` - Advanced table provider registration

**SessionConfig Untested Methods**:
- `with_batch_size()`
- `with_repartition_sorts()`
- `with_repartition_file_scans()`
- `with_repartition_file_min_size()`

**RuntimeEnvBuilder Untested Methods**:
- `with_disk_manager_disabled()`
- `with_unbounded_memory_pool()`
- `with_greedy_memory_pool()`

### 7. Expression Methods

**Location**: `python/datafusion/expr.py`
**Test File**: `python/tests/test_expr.py`

**Untested Methods**:
- `canonical_name()` - Complete string representation
- `variant_name()` - Returns Expr variant name
- `rex_type()` - Returns RexType classification
- `types()` - Returns DataTypeMap
- `python_value()` - Extracts value from literal
- `rex_call_operands()` - Returns operands
- `rex_call_operator()` - Extracts operator
- `column_name()` - Compute output column name

### 8. Plan Methods

**Location**: `python/datafusion/plan.py`
**Test File**: `python/tests/test_plans.py`

**LogicalPlan Untested Methods**:
- `display_indent_schema()` - Print indented schema
- `display_graphviz()` - GraphViz visualization

**ExecutionPlan Untested Methods**:
- `children()` - Get list of child plans
- `display_indent()` - Indented physical plan display

### 9. Common Module Types

**Location**: `python/datafusion/common.py`
**Test File**: None dedicated

**Untested Types**:
- `DFSchema` - No functional tests
- `DataType` - No tests
- `DataTypeMap` - No tests
- `PythonType` - No tests
- `RexType` - No tests
- `SqlFunction`, `SqlSchema`, `SqlStatistics`, `SqlTable`, `SqlType`, `SqlView` - No tests

### 10. Temporal Functions

**Location**: `python/datafusion/functions.py`

**Untested (6 functions)**:
- `current_date()` - Returns current UTC date
- `current_time()` - Returns current UTC time
- `make_date()` - Construct date from year, month, day
- `now()` - Returns current timestamp
- `to_unixtime()` - Convert to Unix time
- `to_hex()` - Integer to hex string

### 11. String Functions

**Location**: `python/datafusion/functions.py`

**Untested (7 functions)**:
- `char_length()` - Alias for length
- `find_in_set()` - Find string in comma-separated list
- `instr()` - Alias for strpos
- `levenshtein()` - Edit distance calculation
- `position()` - Alias for strpos
- `substr_index()` - Substring before N occurrences
- `substring()` - With explicit position and length

---

## Edge Cases Requiring Tests

### Empty DataFrames
- Current: Partially tested for `to_pandas()`, `to_polars()`, `to_arrow_table()`
- Missing: Empty record batch streams, empty aggregation results

### Null Value Handling
- Current: Used in various tests but not dedicated testing
- Missing: All-null batches, null in multi-argument functions, null handling in UDFs

### Type Coercion
- Current: Basic `cast()` test exists
- Missing: Invalid cast operations, implicit type conversions, cross-type comparisons

### Large Datasets
- Current: No performance tests
- Missing: Tests with millions of rows, memory efficiency tests, large batch handling

---

## Recommended New Test Files

1. **`test_aggregation_functions.py`** - All aggregation functions (~22 tests)
2. **`test_window_functions.py`** - All window functions (~15 tests)
3. **`test_object_store.py`** (expand) - All object store types (~20 tests)
4. **`test_expression_builders.py`** - coalesce, nullif, in_list, struct (~15 tests)
5. **`test_common_types.py`** - DFSchema, DataType, etc. (~15 tests)
6. **`test_error_handling.py`** - Cross-module error cases (~30 tests)
7. **`test_edge_cases.py`** - Empty, null, large datasets (~25 tests)

---

## Summary of Test Improvements Needed

| Priority | Area | Estimated Tests Needed |
|----------|------|----------------------|
| CRITICAL | Aggregation Functions | 25+ |
| CRITICAL | Window Functions | 20+ |
| HIGH | Object Store | 20+ |
| HIGH | UDF/UDAF/UDWF Gaps | 30+ |
| HIGH | Error Handling | 30+ |
| MEDIUM | SessionContext Methods | 15+ |
| MEDIUM | Expression Methods | 10+ |
| MEDIUM | Common Types | 15+ |
| MEDIUM | Temporal Functions | 10+ |
| LOW | String Function Aliases | 5+ |
| LOW | Edge Cases | 25+ |
| **TOTAL** | | **~200+ tests** |

---

## Quick Wins

These tests can be added with minimal effort:

1. **Aggregation basics**: Add tests for `sum()`, `count()`, `avg()`, `min()`, `max()` - these are one-liners
2. **Window function basics**: Add tests for `row_number()`, `rank()`, `dense_rank()`
3. **SessionContext.session_id()**: Simple property access test
4. **UDF with multiple arguments**: Extend existing test patterns
5. **Volatility enum values**: Add parametrized tests to existing UDF tests

---

## Conclusion

While the datafusion-python test suite provides good coverage for many core features, there are critical gaps in:

1. **SQL aggregation functions** - The most commonly used SQL operations
2. **Window functions** - Entire category with zero coverage
3. **Object stores** - Critical for cloud deployments
4. **Error handling** - Essential for production reliability

Addressing these gaps would significantly improve the reliability and maintainability of the project.
