# RustScrape — Development Description

## 1. Project Overview

RustScrape is a configurable, general-purpose recursive web crawler and data extraction tool written in Rust.

The application allows users to provide starting URLs, define crawling boundaries, configure extraction rules, and export collected information into structured formats such as JSON and CSV.

The system is designed to separate web crawling, data extraction, data processing, and output generation into independent modules.

### Primary Objectives

- Recursively discover and crawl webpages.
- Restrict crawling to user-defined domains and URL paths.
- Extract structured information from HTML documents.
- Support configurable CSS selectors for targeted extraction.
- Automatically identify common HTML structures such as tables, headings, lists, and metadata.
- Normalize and organize extracted information.
- Export collected data into JSON, CSV, and other supported formats.
- Support asynchronous requests and controlled concurrency.
- Provide reliable error handling and resumable crawling.

### Project Scope

**Included:**

- Static HTML websites.
- Recursive hyperlink discovery.
- Configurable crawling depth and page limits.
- URL normalization and duplicate detection.
- CSS selector-based extraction.
- Automatic HTML table and metadata extraction.
- Structured data serialization.
- Configurable output formats.
- Logging and error reporting.

**Outside the initial scope:**

- JavaScript-heavy website rendering.
- CAPTCHA solving or authentication bypassing.
- Automated login and session management.
- AI-based semantic extraction.
- GUI-based configuration.
- Distributed crawling across multiple machines.

These may be introduced as independent extensions in future versions.

---

# 2. Project Structure

```text
rustscrape/
│
├── Cargo.toml
├── README.md
├── devdescription.md
│
├── configs/
│   └── example.yaml
│
├── output/
│
└── src/
    ├── main.rs
    ├── config.rs
    │
    ├── crawler/
    │   ├── mod.rs
    │   ├── queue.rs
    │   ├── fetcher.rs
    │   ├── parser.rs
    │   ├── scope.rs
    │   └── dedup.rs
    │
    ├── extractor/
    │   ├── mod.rs
    │   ├── selector.rs
    │   ├── tables.rs
    │   ├── metadata.rs
    │   └── automatic.rs
    │
    ├── processor/
    │   ├── mod.rs
    │   └── normalize.rs
    │
    └── exporter/
        ├── mod.rs
        ├── json.rs
        ├── csv.rs
        └── html.rs
```

---

# 3. Dependencies

The following crates are intended for the initial implementation.

| Dependency | Purpose | Used In |
|---|---|---|
| `tokio` | Asynchronous runtime and task management | main, crawler |
| `reqwest` | HTTP requests and webpage fetching | fetcher |
| `scraper` | HTML parsing and CSS selector queries | parser, extractor |
| `url` | URL parsing, resolution and normalization | crawler |
| `serde` | Serialization and deserialization | config, processor, exporter |
| `serde_json` | JSON serialization | exporter |
| `serde_yaml` | YAML configuration parsing | config |
| `csv` | CSV generation | exporter |
| `clap` | Command-line argument parsing | main |
| `tracing` | Structured logging | All modules |
| `tracing-subscriber` | Logging configuration | main |
| `anyhow` | Application-level error handling | All modules |
| `thiserror` | Custom error types | Modules defining errors |
| `futures` | Async stream and concurrency utilities | crawler |

Additional dependencies can be introduced when required.

### Development Dependencies

| Dependency | Purpose |
|---|---|
| `tokio-test` | Testing asynchronous functions |
| `mockito` | Mock HTTP server for testing requests |
| `tempfile` | Temporary files for exporter tests |

### Dependency Guidelines

- Prefer established Rust crates over custom implementations when appropriate.
- Avoid introducing dependencies for functionality that can be implemented simply using existing crates.
- Keep dependencies isolated to modules that actually require them.
- Use asynchronous operations for network requests.
- Avoid blocking the Tokio runtime with synchronous I/O.
- Pin compatible dependency versions through `Cargo.toml` and maintain `Cargo.lock`.

---

# 4. Module Responsibilities

## 4.1 Application Root

### `src/main.rs`

**Role:** Application entry point.

Responsibilities:

- Parse command-line arguments.
- Load the configuration.
- Initialize logging.
- Initialize application state and shared HTTP client.
- Start the crawling process.
- Coordinate extraction, processing and exporting.
- Handle top-level application errors.

It should not contain crawling, parsing or extraction implementation logic.

### `src/config.rs`

**Role:** Configuration management.

Responsibilities:

- Define application configuration structures.
- Load configuration from YAML files.
- Deserialize configuration using Serde.
- Validate configuration values.
- Provide defaults for optional settings.
- Define structures for crawling, extraction and output settings.

Configuration should be represented using strongly typed Rust structures.

---

## 4.2 Crawler Module

The crawler module is responsible for discovering, fetching and managing webpages.

It must not be responsible for deciding what information should be extracted or how that information should be exported.

### `src/crawler/mod.rs`

**Role:** Crawler orchestration.

Responsibilities:

- Coordinate the crawler components.
- Maintain the crawling workflow.
- Retrieve URLs from the queue.
- Check crawling restrictions.
- Dispatch requests to the fetcher.
- Pass fetched documents to the parser.
- Submit newly discovered URLs to the queue.
- Send eligible pages to the extraction pipeline.

**Imports:**

- `queue`
- `fetcher`
- `parser`
- `scope`
- `dedup`
- Configuration structures.
- Shared HTTP client.

**Exports:**

- Main crawler interface.
- Crawling result structures.
- Public crawler functions.

### `src/crawler/queue.rs`

**Role:** URL scheduling and queue management.

Responsibilities:

- Maintain pending URLs.
- Track crawling depth.
- Store URL metadata required for scheduling.
- Provide URLs to crawler workers.
- Support asynchronous access when concurrency is enabled.

A queued URL should contain at least:

- URL.
- Crawling depth.
- Parent URL or discovery source.

**Imports:**

- URL types.
- Synchronization primitives from Tokio or the standard library.

**Exports:**

- Queue structure.
- Queue item structure.
- Queue management methods.

### `src/crawler/fetcher.rs`

**Role:** HTTP communication.

Responsibilities:

- Send HTTP GET requests.
- Retrieve webpage content.
- Handle HTTP status codes.
- Handle timeouts and network errors.
- Follow permitted redirects.
- Return response metadata and HTML content.
- Apply request headers and user-agent configuration.

**Imports:**

- `reqwest`
- `anyhow` or custom error types.
- `tracing`
- URL types.

**Exports:**

- Fetcher interface.
- Fetched page structure.
- Fetching functions.

The HTTP client should be reused rather than recreated for every request.

### `src/crawler/parser.rs`

**Role:** HTML document parsing and hyperlink discovery.

Responsibilities:

- Parse fetched HTML.
- Extract hyperlinks.
- Resolve relative URLs against the current page URL.
- Identify links suitable for further crawling.
- Extract basic page metadata where required.
- Return discovered URLs to the crawler.

This module should discover links, not decide whether they are permitted by the crawling scope.

**Imports:**

- `scraper`
- `url`
- Fetched page structures.

**Exports:**

- Parsed document structures.
- Discovered link collections.
- HTML parsing functions.

### `src/crawler/scope.rs`

**Role:** Crawling boundary enforcement.

Responsibilities:

- Validate discovered URLs against configured scope.
- Check allowed domains.
- Check allowed URL paths and patterns.
- Reject unsupported schemes.
- Enforce maximum crawling depth.
- Enforce maximum page limits.
- Prevent crawling outside the user's explicitly permitted scope.

**Imports:**

- `url`
- Configuration structures.
- Scope-related pattern matching utilities if required.

**Exports:**

- Scope validation structure.
- URL validation methods.
- Scope-related error or rejection information.

### `src/crawler/dedup.rs`

**Role:** Duplicate URL detection.

Responsibilities:

- Maintain a record of discovered URLs.
- Normalize URLs before comparison.
- Prevent repeated scheduling and fetching.
- Distinguish between discovered, queued and successfully processed URLs where necessary.

**Imports:**

- `url`
- Standard collections such as `HashSet`.
- Synchronization primitives if shared between workers.

**Exports:**

- Deduplication structure.
- URL registration and lookup methods.

URL normalization must be conservative. Query parameters should not be discarded automatically because they may identify different pages.

---

## 4.3 Extractor Module

The extractor module processes HTML documents and retrieves information according to configured extraction rules.

It must remain independent of the crawling mechanism.

### `src/extractor/mod.rs`

**Role:** Extraction orchestration.

Responsibilities:

- Receive parsed webpages.
- Select the appropriate extraction strategy.
- Execute configured extraction rules.
- Combine extraction results.
- Produce structured records.

**Imports:**

- `selectors`
- `tables`
- `metadata`
- `automatic`
- Configuration structures.
- Parsed document structures.

**Exports:**

- Main extraction interface.
- Extracted record structures.
- Extraction result types.

### `src/extractor/selector.rs`

**Role:** CSS selector-based extraction.

Responsibilities:

- Read user-defined CSS selectors.
- Locate matching HTML elements.
- Extract text and attributes.
- Support multiple matching elements.
- Map extracted values to configured field names.
- Handle missing or empty elements.

Example:

```yaml
fields:
  name:
    selector: "h2.faculty-name"
    attribute: "text"

  email:
    selector: "a.email"
    attribute: "href"
```

**Imports:**

- `scraper`
- Extraction configuration structures.

**Exports:**

- Selector extraction functions.
- Field extraction results.

### `src/extractor/tables.rs`

**Role:** HTML table extraction.

Responsibilities:

- Detect HTML tables.
- Extract table headers.
- Extract table rows and cells.
- Associate cells with their corresponding headers.
- Handle tables without headers.
- Preserve table structure in the output.

**Imports:**

- `scraper`
- Structured record types.

**Exports:**

- Table extraction functions.
- Table data structures.

### `src/extractor/metadata.rs`

**Role:** Page metadata extraction.

Responsibilities:

- Extract HTML title.
- Extract meta descriptions.
- Extract Open Graph metadata.
- Extract canonical URLs.
- Extract JSON-LD structured data where available.

**Imports:**

- `scraper`
- `serde_json`
- URL types where necessary.

**Exports:**

- Metadata structures.
- Metadata extraction functions.

### `src/extractor/automatic.rs`

**Role:** Automatic content extraction.

Responsibilities:

- Identify common HTML content structures.
- Extract headings and associated text.
- Extract paragraphs and lists.
- Identify repeated content blocks.
- Combine related elements into structured records.
- Support extraction without manually supplied CSS selectors.

This module should use deterministic HTML analysis initially.

AI-based semantic extraction is outside the initial implementation scope.

**Imports:**

- `scraper`
- Table extraction functionality.
- Metadata extraction functionality.
- Structured record types.

**Exports:**

- Automatic extraction functions.
- Automatically detected content structures.

---

## 4.4 Processor Module

The processor module cleans and organizes extracted information before export.

### `src/processor/mod.rs`

**Role:** Data processing orchestration.

Responsibilities:

- Receive extraction results.
- Apply configured processing operations.
- Prepare final records for export.

**Imports:**

- `normalize`
- Extraction result structures.
- Configuration structures.

**Exports:**

- Processing interface.
- Final processed record structures.

### `src/processor/normalize.rs`

**Role:** Data normalization and cleanup.

Responsibilities:

- Trim unnecessary whitespace.
- Normalize text formatting.
- Remove empty values where appropriate.
- Normalize extracted URLs.
- Handle duplicate records when configured.
- Convert extracted values into consistent data structures.

Normalization should preserve the original extracted values whenever possible.

**Imports:**

- Standard string and collection utilities.
- URL types.
- Structured record types.

**Exports:**

- Normalization functions.
- Cleaned data structures.

---

## 4.5 Exporter Module

The exporter module converts processed records into user-requested output formats.

It should not perform crawling or HTML extraction.

### `src/exporter/mod.rs`

**Role:** Output format selection and orchestration.

Responsibilities:

- Identify the requested output format.
- Dispatch records to the appropriate exporter.
- Manage output paths.
- Handle export errors.

**Imports:**

- `json`
- `csv`
- `html`
- Processed record structures.
- Output configuration structures.

**Exports:**

- Export interface.
- Output result structures.

### `src/exporter/json.rs`

**Role:** JSON output generation.

Responsibilities:

- Serialize processed records.
- Preserve nested data structures.
- Support pretty-printed JSON.
- Write results to the configured output path.

**Imports:**

- `serde`
- `serde_json`
- Processed record structures.

**Exports:**

- JSON serialization and writing functions.

### `src/exporter/csv.rs`

**Role:** CSV output generation.

Responsibilities:

- Convert structured records into tabular rows.
- Write column headers.
- Handle missing values.
- Serialize nested values using a consistent representation.
- Write CSV files.

**Imports:**

- `csv`
- Processed record structures.

**Exports:**

- CSV writing functions.

### `src/exporter/html.rs`

**Role:** Human-readable HTML report generation.

Responsibilities:

- Generate HTML tables from processed records.
- Display extracted information in a readable format.
- Include source page references.
- Produce standalone HTML reports.

**Imports:**

- Processed record structures.
- Standard formatting and file-writing utilities.

**Exports:**

- HTML report generation functions.

---

# 5. Data Flow

The application should follow this processing sequence:

```text
User Input
    |
    v
Configuration Loader
    |
    v
Crawler Initialization
    |
    v
URL Queue
    |
    v
Scope Validation
    |
    v
HTTP Fetcher
    |
    v
HTML Parser
    |
    +------> Discover Links
    |             |
    |             v
    |        Deduplication
    |             |
    |             v
    |        URL Queue
    |
    v
Extraction Engine
    |
    v
Data Processor
    |
    v
Exporter
    |
    v
JSON / CSV / HTML
```

The crawler should continue discovering and processing eligible URLs until the queue is empty or a configured crawling limit is reached.

---

# 6. Configuration Design

The application should use YAML as its initial configuration format.

Example:

```yaml
start_urls:
  - "https://example.edu/faculty"

scope:
  allowed_domains:
    - "example.edu"

  allowed_paths:
    - "/faculty"

  max_depth: 3
  max_pages: 500

  respect_robots_txt: true

crawler:
  concurrency: 5
  request_timeout: 15
  delay_ms: 500
  user_agent: "RustScrape/0.1"

extraction:
  mode: automatic

  collect:
    - headings
    - paragraphs
    - lists
    - tables
    - metadata
    - links

  fields:
    name:
      selector: "h2.faculty-name"

    research_areas:
      selector: ".research-interests"

processing:
  normalize_whitespace: true
  remove_empty_values: true
  deduplicate_records: true

output:
  format: json
  directory: "./output"
  filename: "results"
  pretty_print: true
```

All configuration options should be validated before crawling begins.

The application should provide useful error messages for invalid configurations.

---

# 7. Core Data Structures

The application should use strongly typed structures wherever possible.

Suggested structures:

### CrawlTarget

Represents a URL scheduled for crawling.

Fields:

- `url`
- `depth`
- `parent_url`

### FetchedPage

Represents a successfully fetched webpage.

Fields:

- `url`
- `status_code`
- `content_type`
- `html`
- `final_url`

### ExtractedRecord

Represents information extracted from a webpage.

Fields:

- `source_url`
- `page_title`
- `data`

The `data` field should support flexible structured values using `serde_json::Value` or an equivalent representation.

### CrawlResult

Represents the overall crawling result.

Fields:

- `pages_discovered`
- `pages_fetched`
- `pages_failed`
- `records_extracted`
- `records`
- `errors`

These structures should be defined in appropriate modules or a dedicated shared types module if they become widely used.

---

# 8. Development Guidelines

## Architecture

- Maintain clear separation between crawling, extraction, processing and exporting.
- Avoid placing implementation logic in `main.rs`.
- Avoid circular module dependencies.
- Keep public interfaces minimal.
- Prefer passing configuration and shared state explicitly.
- Use traits when multiple implementations of a component are genuinely required.

## Performance

- Use Tokio for asynchronous networking.
- Reuse HTTP clients.
- Implement bounded concurrency.
- Avoid unnecessary cloning of large HTML documents.
- Avoid fetching duplicate URLs.
- Do not introduce unbounded task spawning.

## Reliability

- Handle network errors without terminating the entire crawl.
- Use request timeouts.
- Implement controlled retries for temporary failures.
- Respect HTTP status codes and server restrictions.
- Record failed URLs for debugging.
- Keep crawling limits configurable.

## Safety and Crawling Restrictions

- Respect robots.txt.
- Apply configurable request delays and concurrency limits.
- Restrict crawling to explicitly permitted domains and paths.
- Do not attempt to bypass authentication, CAPTCHAs or access restrictions.
- Avoid repeatedly requesting the same resource.
- Validate redirects against the configured scope before following them.

## Code Quality

- Use Rust's type system to prevent invalid states where practical.
- Document public functions and structures.
- Use `Result` for fallible operations.
- Avoid unnecessary `.unwrap()` calls in production code.
- Add unit tests for parsing, normalization, scope validation and extraction.
- Add integration tests using mock HTTP servers.

---

# 9. Development Roadmap

Development should proceed incrementally.

| Phase | Implementation | Expected Result |
|---|---|---|
| 1 | HTTP fetcher and HTML parser | Fetch and parse individual pages |
| 2 | URL discovery and queue | Discover linked pages |
| 3 | Scope and deduplication | Controlled recursive crawling |
| 4 | CSS selector extraction | Extract configured fields |
| 5 | Table and metadata extraction | Automatically collect structured HTML data |
| 6 | Data processing | Normalize and organize records |
| 7 | JSON and CSV exporters | Generate usable output |
| 8 | Async concurrency | Improve crawling performance |
| 9 | Resume support | Continue interrupted crawls |
| 10 | Advanced extraction | Automatic content grouping and browser rendering |

Each phase should be tested before moving to the next.

---

# 10. Initial Development Target

The first working version should be capable of:

1. Accepting a YAML configuration file through the CLI.
2. Fetching a starting webpage.
3. Discovering and following links within the permitted scope.
4. Respecting maximum depth and page limits.
5. Avoiding duplicate requests.
6. Extracting page titles, headings, paragraphs and HTML tables.
7. Extracting user-defined fields through CSS selectors.
8. Organizing the extracted data into structured records.
9. Exporting the results into JSON and CSV.

The initial implementation should prioritize correctness, modularity and predictable behavior over advanced automation or maximum performance.

The architecture should allow future additions such as browser-based rendering, AI-assisted extraction, additional export formats and persistent crawl storage without requiring a major redesign.

---

# 11. Learning-Oriented Implementation Workflow

Build this project in small steps while learning Rust. For each step, be able to explain the input, output, module responsibility, and possible failures before expanding the feature.

For each milestone:

1. Describe the behavior in plain language and give a small example input and output.
2. Identify the module that owns the behavior and the data types it needs.
3. Implement only that behavior; keep `main.rs` as the wiring point.
4. Study compiler diagnostics to understand ownership, borrowing, and type constraints.
5. Review edge cases and failures, then write focused tests before moving on.

Recommended order:

| Step | Small outcome | Rust concepts to practice |
|---|---|---|
| A | Represent a crawl target and fetched page | Structs, field types, visibility, module paths |
| B | Fetch one page and return response information | `Result`, error propagation, reusable client ownership |
| C | Parse one HTML document and collect its title and links | Borrowing, iterators, parsing errors, URL resolution |
| D | Decide whether a discovered URL is allowed | Pure functions, rejection reasons, URL semantics |
| E | Track discovered URLs and schedule them without duplicates | `HashSet`, `HashMap`, collection invariants |
| F | Extract configured fields from a document | Configuration types, validation, optional values |
| G | Normalize records and export one format | Serde, trait bounds, I/O errors |
| H | Connect the pieces into a sequential crawl | Ownership across module boundaries, orchestration |
| I | Add bounded async concurrency | Tokio tasks, shared state, synchronization, cancellation |

Keep the first crawler sequential. This makes queue behavior, scope enforcement, duplicate handling, and failure policy easier to reason about. Add concurrency after those rules are clear and independently reviewable.

Prefer errors that preserve useful context. A malformed CSS selector, an invalid configuration value, and a temporary network failure have different causes. Avoid designing a large custom error hierarchy before module boundaries and real failure cases are known.

Treat tests as executable examples of module behavior. Start with pure logic such as URL scope decisions, normalization, and HTML parsing; use a local mock server for HTTP behavior when that layer is introduced. Cover normal input, missing data, and at least one failure case.

Keep module interfaces minimal: expose only what neighboring modules need, rather than making every implementation detail public.
