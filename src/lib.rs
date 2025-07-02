//! # Minimal Agent Framework
//!
//! Core traits for building agent systems

/// An autonomous agent that can process information
pub trait Agent {
    /// Returns the agent's role/function
    fn role(&self) -> &str;
    
    /// Processes input and optionally uses tools
    fn process(&self, input: &str, tools: &[&dyn Tool]) -> Result<String, String>;
}

/// A tool that agents can use
pub trait Tool {
    /// Unique identifier for the tool
    fn name(&self) -> &str;
    
    /// What the tool does
    fn description(&self) -> &str;
    
    /// Executes the tool's function
    fn execute(&self, input: &str) -> Result<String, String>;
}

/// Persistent storage for agent interactions
pub trait Memory {
    /// Stores a message with context
    fn append(&mut self, agent_role: &str, message: &str) -> Result<(), String>;
    
    /// Retrieves all stored messages
    fn read(&self) -> Result<String, String>;
    
    /// Clears the memory
    fn clear(&mut self) -> Result<(), String>;
}

/// Coordinates multiple agents working together
pub trait Crew {
    /// Executes a task by passing it through all agents
    fn execute(&mut self, initial_input: &str) -> Result<String, String>;
}

/// Error type for execution failures
#[derive(Debug)]
pub struct ExecutionError(pub String);