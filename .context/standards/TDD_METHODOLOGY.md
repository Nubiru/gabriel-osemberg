# TDD Methodology

## Core Principle

> **Test-Driven Development**: Write tests first (RED), then implement the function to make tests pass (GREEN), then refactor while keeping tests green (REFACTOR). Tests validate behavior, not implementation.

---

## Framework

<!-- CUSTOMIZE: Replace with your project's test framework and commands. -->

**Test framework**: `{TEST_FRAMEWORK}`
**Location**: `{TEST_DIR}/`
**Build & run**: `{TEST_COMMAND}`

### Common Frameworks by Stack

| Stack | Framework | Command |
|-------|-----------|---------|
| Python | pytest | `pytest` |
| TypeScript | Jest / Vitest | `npm test` / `npx vitest` |
| Rust | built-in | `cargo test` |
| Go | built-in | `go test ./...` |
| Java | JUnit | `mvn test` / `gradle test` |
| C | Unity / CUnit | `make test` |

---

## TDD Cycle

### 1. RED -- Write a failing test

Write a test that describes the behavior you want. Run it. Watch it fail. This confirms:
- The test is actually testing something
- The feature does not accidentally already exist

```
// Pseudocode -- adapt to your language
test "calculate_total applies discount" {
    // Arrange
    items = [Item(price=100), Item(price=50)]
    discount = 0.10

    // Act
    total = calculate_total(items, discount)

    // Assert
    expect(total).to_equal(135.0)
}
```

### 2. GREEN -- Write minimal code to pass

Implement the simplest code that makes the test pass. Do not over-engineer. Do not handle edge cases yet. Just make the test green.

### 3. REFACTOR -- Improve while green

With the safety net of passing tests:
- Extract constants and helper functions
- Improve variable names
- Remove duplication
- Add edge case tests (which creates new RED-GREEN cycles)

**Important**: Run tests after every refactor step. If a test fails, you broke something -- undo and try again.

---

## Test Organization

<!-- CUSTOMIZE: Adapt to your project structure. -->

```
{TEST_DIR}/
  {DOMAIN_A}/           Domain A tests
    test_{module_1}
    test_{module_2}
  {DOMAIN_B}/           Domain B tests
    test_{module_3}
  integration/          Integration tests (cross-module)
    test_{flow_1}
  fixtures/             Shared test data
  helpers/              Shared test utilities
```

**Mirror your source structure**: Every source module should have a corresponding test module in the same relative path under the test directory.

---

## Test Naming

```
test_{function_name}_{scenario}

Examples:
test_calculate_total_empty_cart
test_calculate_total_with_discount
test_calculate_total_negative_price_raises_error
test_parse_date_valid_iso_format
test_parse_date_invalid_string_returns_none
test_authenticate_expired_token_returns_unauthorized
```

**Name what you are testing and under what conditions.** A test name should read like a specification.

---

## Test Structure: AAA Pattern

Every test follows **Arrange, Act, Assert**:

```
test "description of expected behavior" {
    // Arrange -- set up inputs and expected outputs
    input = ...
    expected = ...

    // Act -- call the function under test
    result = function_under_test(input)

    // Assert -- verify the result matches expectations
    assert result == expected
}
```

**One assertion per test** (prefer, not strict). Multiple assertions are acceptable when they validate different aspects of a single behavior.

---

## What to Test

| Layer | What to test | How |
|-------|-------------|-----|
| Pure Core / Domain | Function correctness | Known input/output pairs, edge cases, boundary values |
| Services / Use Cases | Orchestration logic | Mock dependencies, verify correct calls |
| Infrastructure | Integration with external systems | Integration tests with real/test database |
| API / Handlers | Request/response contracts | HTTP test client, schema validation |
| UI Components | Rendering and interaction | Component tests, snapshot tests |

---

## What NOT to Test

- **Third-party library internals** (trust the library, test your usage of it)
- **Language features** (do not test that `if` works)
- **Private implementation details** (test behavior, not internal structure)
- **Visual appearance** (verify by eye or with visual regression tools, not unit tests)
- **Configuration values** (test the logic that uses config, not the config itself)

---

## Test Quality Rules

### 1. Tests must be deterministic

No flaky tests. If a test depends on time, use a fixed clock. If it depends on randomness, use a fixed seed. If it depends on network, mock the network.

### 2. Tests must be independent

Each test sets up its own state and cleans up after itself. No test depends on another test running first. Tests can run in any order.

### 3. Tests must be fast

Unit tests should run in milliseconds. If a test takes more than 1 second, it is probably an integration test -- move it to the integration suite.

### 4. Tests must be readable

A test is documentation. Someone reading the test should understand what the function does without reading the implementation.

---

## Running Tests

```bash
# Run all tests
{TEST_COMMAND}

# Run a specific test file
{TEST_SPECIFIC_COMMAND}

# Run tests matching a pattern
{TEST_PATTERN_COMMAND}

# Run with verbose output
{TEST_VERBOSE_COMMAND}
```

<!-- CUSTOMIZE EXAMPLES:
Python:  pytest, pytest tests/test_auth.py, pytest -k "test_login", pytest -v
TS:      npm test, npx vitest run tests/auth.test.ts, npx vitest -t "login", npm test -- --verbose
Rust:    cargo test, cargo test --test auth, cargo test login, cargo test -- --nocapture
Go:      go test ./..., go test ./internal/auth/, go test -run TestLogin, go test -v ./...
-->

All tests must pass with zero warnings/errors before committing.
