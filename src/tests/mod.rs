#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nft_creation() {
        let name = "Test NFT".to_string();
        let result = create_nft(name.clone()).await;
        assert!(result.is_ok());
        
        let nft_id = result.unwrap();
        let nft = get_nft(nft_id).unwrap();
        
        assert_eq!(nft.name, name);
        assert_eq!(nft.interaction_count, 0);
        assert_eq!(nft.growth_level, 1);
    }

    #[tokio::test]
    async fn test_interaction() {
        let name = "Interactive NFT".to_string();
        let nft_id = create_nft(name).await.unwrap();
        
        let input = "Hello!".to_string();
        let response = interact(nft_id, input).await.unwrap();
        
        assert!(response.message.contains("Count: 1"));
    }
}