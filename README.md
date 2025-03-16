# RustEdge Agent

**RustEdge Agent** is a Rust tool designed to help developers troubleshoot and fix Rust error messages. Currently, it uses a static JSON database to suggest solutions based on error codes extracted from messages (e.g., "error[E0425]: cannot find function"). The project is open-source and actively being developed, with plans to integrate AI for smarter, context-aware suggestions.

---

## Features
- **Error Suggestion**: Enter a Rust error message, and the agent will suggest a fix if the error code is recognized.
- **Multi-Error Support**: Handles multiple error codes in a single message and provides suggestions for each.
- **User Contributions**: If an error code isn’t recognized, you can add your own suggestion to the database.
- **Future AI Integration**: Work is in progress to add AI-powered suggestions using a pre-trained model for more intelligent fixes.

---

## Installation
To get started with RustEdge Agent, follow these steps:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/Samir-spec-star/RustEdge-Agent.git
2. Navigate to the Project Directory:
   ```bash
   cd rustedge-agent
3. Build the Project
   ```bash
   cargo build
5. Run the Agnet
   ```bash
   cargo run
   
