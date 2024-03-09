use std::{collections::HashMap, path::PathBuf, sync::Arc};

use serde_json::json;
use serenity::all::ChannelType;

use crate::{
    slack_export::{
        channel::ChannelInfo,
        user::{User, UserId},
    },
    Context,
};

pub struct MasterLoader {
    user_map: Arc<HashMap<UserId, User>>,
    channel_loader_map: HashMap<String, ChannelLoader>,
}

impl MasterLoader {
    pub async fn new(context: Context) -> anyhow::Result<Self> {
        let user_map = Arc::new(Self::create_user_map(context.root.clone()).await);

        println!("Found {} users in the server", user_map.len());

        let channel_loader_map =
            Self::create_channel_loader_map(user_map.clone(), Arc::new(context)).await?;

        println!("Found {} channels to load", channel_loader_map.len());

        Ok(MasterLoader {
            user_map,
            channel_loader_map,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let results = self
            .channel_loader_map
            .into_values()
            .map(|channel_loader| tokio::spawn(async move { channel_loader.run().await }))
            .collect::<Vec<_>>();

        for result in results {
            result.await??;
        }

        Ok(())
    }

    async fn create_user_map(root: PathBuf) -> HashMap<UserId, User> {
        let file_path = root.join("users.json");
        let file = tokio::fs::read(file_path).await.unwrap();
        let users: Vec<User> = serde_json::from_slice(&file).unwrap();

        users
            .into_iter()
            .map(|user| (user.id.clone(), user))
            .collect()
    }

    async fn create_channel_loader_map(
        user_map: Arc<HashMap<UserId, User>>,
        context: Arc<Context>,
    ) -> anyhow::Result<HashMap<String, ChannelLoader>> {
        let file_path = context.root.join("channels.json");
        let file = tokio::fs::read(file_path).await?;
        let channels: Vec<ChannelInfo> = serde_json::from_slice(&file)?;

        Ok(channels
            .into_iter()
            .map(|channel_info| {
                (
                    channel_info.id.clone(),
                    ChannelLoader::new(user_map.clone(), channel_info, context.clone()),
                )
            })
            .collect())
    }
}

struct ChannelLoader {
    user_map: Arc<HashMap<UserId, User>>,
    serenity_http: serenity::http::Http,
    channel_info: ChannelInfo,
    context: Arc<Context>,
}

impl ChannelLoader {
    pub fn new(
        user_map: Arc<HashMap<UserId, User>>,
        channel_info: ChannelInfo,
        context: Arc<Context>,
    ) -> Self {
        ChannelLoader {
            user_map,
            serenity_http: serenity::http::Http::new(&context.discord_token),
            channel_info,
            context,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        self.create_channel().await?;

        Ok(())
    }

    async fn create_channel(&self) -> anyhow::Result<()> {
        let already_exists = self
            .serenity_http
            .get_channels(self.context.guild_id)
            .await?
            .iter()
            .any(|channel| channel.name == self.channel_info.name);

        match (already_exists, self.context.use_existing_channel) {
            (false, _) => {
                self.serenity_http
                    .create_channel(
                        self.context.guild_id,
                        &json!({
                            "name": self.channel_info.name,
                            "type": ChannelType::Text,
                            "topic": self.channel_info.topic.value,
                        }),
                        None,
                    )
                    .await?;
                println!("Created channel {}", self.channel_info.name);
            }
            (true, true) => {
                println!("Using existing channel {}", self.channel_info.name);
            }
            (true, false) => {
                return Err(anyhow::anyhow!(
                    "Channel {} already exists",
                    self.channel_info.name
                ));
            }
        }

        Ok(())
    }
}
