use std::sync::Arc;

use futures::{Future, TryStreamExt};
use reth_execution_types::Chain;
use reth_exex::{ExExContext, ExExEvent};
use reth_node_api::{FullNodeComponents, NodeTypes};
use reth_primitives::{Block, EthPrimitives};
use reth_tracing::tracing::{info, warn};

/// The witness generation ExEx.
#[derive(Debug)]
pub struct WitnessGeneratorExEx<Node: FullNodeComponents> {
    ctx: ExExContext<Node>,
}

impl<Node> WitnessGeneratorExEx<Node>
where
    Node: FullNodeComponents<Types: NodeTypes<Primitives = EthPrimitives>>,
{
    /// Create a new instance of the ExEx.
    fn new(ctx: ExExContext<Node>) -> Self {
        Self { ctx }
    }

    /// The main loop of the ExEx.
    async fn start(mut self) -> eyre::Result<()> {
        while let Some(notification) = self.ctx.notifications.try_next().await? {
            // For witness generation, we are only interested in new blocks that have been
            // committed to the chain. A reorg is handled as a commit of a new chain.
            if let Some(committed_chain) = notification.committed_chain() {
                info!(
                    committed_chain = ?committed_chain.range(),
                    "Processing committed chain for witness generation"
                );
                self.handle_chain_committed(committed_chain.clone()).await?;

                self.ctx
                    .events
                    .send(ExExEvent::FinishedHeight(committed_chain.tip().num_hash()))?;
            }
        }
        Ok(())
    }

    /// Handle a new committed chain.
    async fn handle_chain_committed(&self, chain: Arc<Chain>) -> eyre::Result<()> {
        // Process each block in the committed chain
        for (block_number, block) in chain.blocks() {
            // The block from the chain is a `RecoveredBlock`, we need to convert it to a `Block`
            let block = block.clone().into_block();
            if let Err(e) = self.generate_witness_with_reth_providers(&block).await {
                warn!("Failed to generate witness for block {}: {}", block_number, e);
                // Continue processing other blocks even if one fails
            }
        }
        Ok(())
    }

    /// Generate witness for a block using reth's internal providers
    async fn generate_witness_with_reth_providers(&self, block: &Block) -> eyre::Result<()> {
        info!("Generating witness for block {} using reth providers", block.number);

        // TODO: Implement the actual witness generation logic using reth's providers
        // let provider = self.ctx.provider_factory.provider()?;
        // ...

        // For now, just log that we would generate a witness
        info!(
            "Would generate witness for block {} with {} transactions",
            block.number,
            block.body.transactions.len()
        );

        Ok(())
    }

    /// Send witness to proving service (placeholder)
    #[allow(dead_code)]
    async fn send_to_proving_service(
        &self,
        _witness: openvm_client_executor::io::ClientExecutorInput,
    ) -> eyre::Result<()> {
        info!("Witness sent to proving service (placeholder)");
        Ok(())
    }
}

/// The initialization logic of the ExEx
pub async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>>
where
    Node: FullNodeComponents<Types: NodeTypes<Primitives = EthPrimitives>>,
{
    Ok(WitnessGeneratorExEx::new(ctx).start())
}

#[cfg(test)]
mod tests {
    use super::*;
    use reth_execution_types::{Chain, ExecutionOutcome};
    use reth_exex_test_utils::{test_exex_context, PollOnce};
    use std::pin::pin;

    #[tokio::test]
    async fn test_witness_exex() -> eyre::Result<()> {
        // Initialize a test Execution Extension context with all dependencies
        let (ctx, mut handle) = test_exex_context().await?;

        // Save the current head of the chain to check the finished height against it later
        let head = ctx.head;

        // Send a notification to the Execution Extension that the chain has been committed
        handle
            .send_notification_chain_committed(Chain::from_block(
                handle.genesis.clone(),
                ExecutionOutcome::default(),
                None,
            ))
            .await?;

        // Initialize the Execution Extension
        let mut exex = pin!(exex_init(ctx).await?);

        // Check that the Execution Extension did not emit any events until we polled it
        handle.assert_events_empty();

        // Poll the Execution Extension once to process incoming notifications
        exex.poll_once().await?;

        // Check that the Execution Extension emitted a `FinishedHeight` event with the correct
        // height
        handle.assert_event_finished_height((head.number, head.hash).into())?;

        Ok(())
    }
}
