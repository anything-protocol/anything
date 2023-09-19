#[cfg(test)]
mod tests {

    use anything_engine::error::EngineResult;
    use anything_graph::flow::{
        action::{ActionBuilder, ActionType, ShellActionBuilder},
        flow,
        node::Node,
    };

    #[tokio::test]
    async fn test_a_very_simple_two_node_flow() -> EngineResult<()> {
        let mut flow = flow::Flow::new();
        let action = ShellActionBuilder::default()
            .command("echo 'ducks'".to_string())
            .build()
            .expect("unable to build action");

        flow.add_node(
            &Node::default(),
            "echo name",
            &ActionBuilder::default()
                .action_type(ActionType::Shell(action))
                .build()
                .unwrap(),
            &vec![],
        )
        .expect("unable to add echo node");

        flow.add_node(
            &Node::default(),
            "print name",
            &ActionBuilder::default()
                .action_type(ActionType::Shell(
                    ShellActionBuilder::default()
                        .command("echo {{name}}".to_string())
                        .build()
                        .expect("echo the name"),
                ))
                .build()
                .unwrap(),
            &vec!["echo name"],
        )
        .expect("unable to add echo node");

        Ok(())
    }
}