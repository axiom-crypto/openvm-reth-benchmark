use exex::WitnessGeneratorExEx;
use reth_node_ethereum::EthereumNode;

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Rollup", move |ctx| async { Ok(WitnessGeneratorExEx::new(ctx).start()) })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
