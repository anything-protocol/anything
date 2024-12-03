use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct TemplateError {
    pub message: String,
    pub variable: String,
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Template error for variable '{}': {}",
            self.variable, self.message
        )
    }
}

impl Error for TemplateError {}

pub struct Templater {
    templates: HashMap<String, Value>,
}

impl Templater {
    pub fn new() -> Self {
        Templater {
            templates: HashMap::new(),
        }
    }

    pub fn add_template(&mut self, name: &str, template: Value) {
        self.templates.insert(name.to_string(), template);
    }

    pub fn get_template_variables(
        &self,
        template_name: &str,
    ) -> Result<Vec<String>, TemplateError> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| TemplateError {
                message: "Template not found".to_string(),
                variable: template_name.to_string(),
            })?;

        self.extract_variables(template)
    }

    fn extract_variables(&self, value: &Value) -> Result<Vec<String>, TemplateError> {
        let mut variables = Vec::new();
        match value {
            Value::Object(map) => {
                for (_, v) in map {
                    variables.extend(self.extract_variables(v)?);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    variables.extend(self.extract_variables(v)?);
                }
            }
            Value::String(s) => {
                let mut start = 0;
                while let Some(open_idx) = s[start..].find("{{") {
                    let open_idx = start + open_idx;
                    let close_idx = s[open_idx..].find("}}").ok_or_else(|| TemplateError {
                        message: "Unclosed template variable".to_string(),
                        variable: s.to_string(),
                    })?;
                    let close_idx = open_idx + close_idx;
                    let variable = s[open_idx + 2..close_idx].trim().to_string();
                    variables.push(variable);
                    start = close_idx + 2;
                }
            }
            _ => {}
        }
        Ok(variables)
    }

    fn get_value_from_path(context: &Value, path: &str) -> Option<Value> {
        let mut current = context;
        let parts: Vec<&str> = path.split('.').collect();

        for (i, part) in parts.iter().enumerate() {
            if let Some(index_start) = part.find('[') {
                let key = &part[..index_start];
                let index_end = part.find(']').unwrap_or(part.len());
                let index: usize = part[index_start + 1..index_end].parse().ok()?;

                current = current.get(key)?;
                if current.is_array() {
                    current = current.get(index)?;
                } else {
                    return None; // Not an array when we expected one
                }
            } else {
                current = current.get(part)?;
            }

            if let Value::String(s) = current {
                if let Ok(parsed) = serde_json::from_str(s) {
                    if i < parts.len() - 1 {
                        // If not the last part, continue traversing
                        return Self::get_value_from_path(&parsed, &parts[i + 1..].join("."));
                    } else {
                        // If it's the last part, return the parsed value
                        return Some(parsed);
                    }
                }
            }
        }
        Some(current.clone())
    }

    pub fn render(&self, template_name: &str, context: &Value) -> Result<Value, TemplateError> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| TemplateError {
                message: "Template not found".to_string(),
                variable: template_name.to_string(),
            })?;

        self.render_value(template, context)
    }

    fn render_value(&self, value: &Value, context: &Value) -> Result<Value, TemplateError> {
        println!("[TEMPLATER] Rendering value: {:?}", value);
        match value {
            Value::Object(map) => {
                println!("[TEMPLATER] Rendering object");
                let mut result = serde_json::Map::new();
                for (k, v) in map {
                    println!("[TEMPLATER] Rendering object key: {}", k);
                    result.insert(k.clone(), self.render_value(v, context)?);
                }
                Ok(Value::Object(result))
            }
            Value::Array(arr) => {
                println!("[TEMPLATER] Rendering array");
                let mut result = Vec::new();
                for (i, v) in arr.iter().enumerate() {
                    println!("[TEMPLATER] Rendering array index: {}", i);
                    result.push(self.render_value(v, context)?);
                }
                Ok(Value::Array(result))
            }
            Value::String(s) => {
                println!("[TEMPLATER] Rendering string: {}", s);

                // Special case: if the string is exactly "{{variables}}" (or any other full variable),
                // return the raw value instead of string conversion
                if s.trim().starts_with("{{") && s.trim().ends_with("}}") {
                    let variable = s.trim()[2..s.trim().len() - 2].trim();
                    if let Some(value) = Self::get_value_from_path(context, variable) {
                        return Ok(value);
                    }
                }

                let mut result = s.clone();
                let mut start = 0;

                while let Some(open_idx) = result[start..].find("{{") {
                    let open_idx = start + open_idx;
                    let close_idx = result[open_idx..].find("}}").ok_or_else(|| TemplateError {
                        message: "Unclosed template variable".to_string(),
                        variable: result.clone(),
                    })?;
                    let close_idx = open_idx + close_idx;
                    let variable = result[open_idx + 2..close_idx].trim();

                    println!("[TEMPLATER] Found variable: {}", variable);

                    let value = Self::get_value_from_path(context, variable).ok_or_else(|| {
                        println!("[TEMPLATER] Variable not found in context: {}", variable);
                        TemplateError {
                            message: "Variable not found in context".to_string(),
                            variable: variable.to_string(),
                        }
                    })?;

                    println!("[TEMPLATER] Variable value: {:?}", value);

                    let replacement = match value {
                        Value::String(s) => s.clone(),
                        _ => value.to_string(),
                    };
                    result.replace_range(open_idx..close_idx + 2, &replacement);
                    start = open_idx + replacement.len();
                }

                println!("[TEMPLATER] Rendered string: {}", result);

                Ok(Value::String(result))
            }
            _ => {
                println!("[TEMPLATER] Returning value as-is: {:?}", value);
                Ok(value.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_object_variable_replacement() {
        let mut templater = Templater::new();

        // Create template with single key-value pair
        let template = json!({
            "greeting": "Hello {{name}}"
        });

        templater.add_template("test_template", template);

        // Create context with replacement value
        let context = json!({
            "name": "World"
        });

        // Render template with context
        let result = templater.render("test_template", &context).unwrap();

        // Check the rendered result
        assert_eq!(
            result,
            json!({
                "greeting": "Hello World"
            })
        );
    }

    #[test]
    fn test_deeply_nested_variable_replacement() {
        let mut templater = Templater::new();

        // Create template that uses deeply nested variable
        let template = json!({
            "message": "Value is: {{data.items[0].details[2].value}}"
        });

        templater.add_template("nested_template", template);

        // Create context with nested data structure
        let context = json!({
            "data": {
                "items": [
                    {
                        "details": [
                            {"value": "first"},
                            {"value": "second"},
                            {"value": "third"},
                            {"value": "fourth"}
                        ]
                    }
                ]
            }
        });

        // Render template with context
        let result = templater.render("nested_template", &context).unwrap();

        // Check the rendered result
        assert_eq!(
            result,
            json!({
                "message": "Value is: third"
            })
        );
    }

    #[test]
    fn test_object_replacement_in_nested_path_inside_string() {
        let mut templater = Templater::new();

        // Create template that uses nested path with object replacement
        let template = json!({
            "message": "Value is: {{data.items[0].details}}"
        });

        templater.add_template("object_template", template);

        // Create context with nested data structure
        let context = json!({
            "data": {
                "items": [
                    {
                        "details": {
                            "id": 123,
                            "name": "test item",
                            "values": ["a", "b", "c"]
                        }
                    }
                ]
            }
        });

        // Render template with context
        let result = templater.render("object_template", &context).unwrap();

        // Check the rendered result includes the full object
        assert_eq!(
            result,
            json!({
                "message": "Value is: {\"id\":123,\"name\":\"test item\",\"values\":[\"a\",\"b\",\"c\"]}"
            })
        );
    }

     #[test]
    fn test_object_replacement_in_nested_path_as_object() {
        let mut templater = Templater::new();

        // Create template that uses nested path with object replacement
        let template = json!({
            "details": "{{data.items[0].details}}"
        });

        templater.add_template("details_object_template", template);

        // Create context with nested data structure
        let context = json!({
            "data": {
                "items": [
                    {
                        "details": {
                            "id": 123,
                            "name": "test item",
                            "values": ["a", "b", "c"]
                        }
                    }
                ]
            }
        });

        // Render template with context
        let result = templater
            .render("details_object_template", &context)
            .unwrap();

        // Check the rendered result includes the full object as an object
        assert_eq!(
            result,
            json!({
                "details": {
                    "id": 123,
                    "name": "test item",
                    "values": ["a", "b", "c"]
                }
            })
        );
    }

    #[test]
    fn test_string_replacement_in_nested_path() {
        let mut templater = Templater::new();

        // Create template that uses nested path with string replacement
        let template = json!({
            "details": "{{data.items[0].name}}"
        });

        templater.add_template("details_string_template", template);

        // Create context with nested data structure
        let context = json!({
            "data": {
                "items": [
                    {
                        "name": "test item"
                    }
                ]
            }
        });

        // Render template with context
        let result = templater
            .render("details_string_template", &context)
            .unwrap();

        // Check the rendered result includes the string value
        assert_eq!(
            result,
            json!({
                "details": "test item"
            })
        );
    }
}