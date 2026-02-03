use qqbot_sdk::{EventData, EventType, TypedEvent};
use serde_json::json;

#[test]
fn parse_c2c_message_event() {
    let payload = json!({
        "id": "5",
        "op": 0,
        "t": "C2C_MESSAGE_CREATE",
        "d": {
            "id": "msg_id",
            "author": {
                "id": "author_id",
                "user_openid": "user_openid_123",
                "union_openid": "union_openid_456"
            },
            "content": "hello",
            "timestamp": "2024-01-01T00:00:00Z",
            "message_type": 0,
            "message_scene": { "source": "default" },
            "msg_seq": 1
        }
    });

    let event = TypedEvent::from_value(&payload).unwrap();
    assert_eq!(event.event_type, EventType::C2cMessageCreate);

    match event.data {
        EventData::C2cMessage(msg) => {
            assert_eq!(msg.author.user_openid, "user_openid_123");
            assert_eq!(msg.content.as_deref(), Some("hello"));
            assert_eq!(msg.author.id.as_deref(), Some("author_id"));
            assert_eq!(msg.author.union_openid.as_deref(), Some("union_openid_456"));
            assert_eq!(msg.message_type, Some(0));
            assert_eq!(
                msg.message_scene.and_then(|scene| scene.source),
                Some("default".to_string())
            );
        }
        other => panic!("unexpected event data: {other:?}"),
    }
}

#[test]
fn parse_message_reaction_event() {
    let payload = json!({
        "op": 0,
        "t": "MESSAGE_REACTION_ADD",
        "d": {
            "user_id": "user_id_1",
            "guild_id": "guild_id_1",
            "channel_id": "channel_id_1",
            "target": { "id": "msg_1", "type": 0 },
            "emoji": { "id": "4", "type": 1 }
        }
    });

    let event = TypedEvent::from_value(&payload).unwrap();
    assert_eq!(event.event_type, EventType::MessageReactionAdd);

    match event.data {
        EventData::MessageReactionAdd(reaction) => {
            assert_eq!(reaction.user_id, "user_id_1");
            assert_eq!(reaction.emoji.id, "4");
        }
        other => panic!("unexpected event data: {other:?}"),
    }
}

#[test]
fn parse_interaction_event() {
    let payload = json!({
        "op": 0,
        "t": "INTERACTION_CREATE",
        "d": {
            "id": "interaction_id_1",
            "type": 11,
            "scene": "c2c",
            "chat_type": 1,
            "timestamp": "2024-01-01T00:00:00Z",
            "data": {
                "type": 1,
                "resolved": { "button_data": "btn_1" }
            }
        }
    });

    let event = TypedEvent::from_value(&payload).unwrap();
    assert_eq!(event.event_type, EventType::InteractionCreate);

    match event.data {
        EventData::InteractionCreate(interaction) => {
            assert_eq!(interaction.id, "interaction_id_1");
            assert_eq!(interaction.data.and_then(|d| d.resolved.and_then(|r| r.button_data)), Some("btn_1".to_string()));
        }
        other => panic!("unexpected event data: {other:?}"),
    }
}

#[test]
fn parse_guild_member_event() {
    let payload = json!({
        "op": 0,
        "t": "GUILD_MEMBER_ADD",
        "d": {
            "guild_id": "guild_1",
            "user": { "id": "user_1" },
            "joined_at": "2024-01-01T00:00:00Z",
            "roles": ["1"],
            "op_user_id": "op_1"
        }
    });

    let event = TypedEvent::from_value(&payload).unwrap();
    assert_eq!(event.event_type, EventType::GuildMemberAdd);

    match event.data {
        EventData::GuildMemberEvent(member_event) => {
            assert_eq!(member_event.member.guild_id.as_deref(), Some("guild_1"));
            assert_eq!(member_event.op_user_id.as_deref(), Some("op_1"));
            assert_eq!(member_event.member.user.and_then(|u| u.id), Some("user_1".to_string()));
        }
        other => panic!("unexpected event data: {other:?}"),
    }
}

#[test]
fn parse_forum_thread_event() {
    let payload = json!({
        "op": 0,
        "t": "FORUM_THREAD_CREATE",
        "d": {
            "guild_id": "guild_1",
            "channel_id": "channel_1",
            "author_id": "author_1",
            "thread_info": {
                "thread_id": "thread_1",
                "title": "Hello",
                "custom_field": 42
            }
        }
    });

    let event = TypedEvent::from_value(&payload).unwrap();
    assert_eq!(event.event_type, EventType::ForumThreadCreate);

    match event.data {
        EventData::ForumThreadEvent(thread_event) => {
            assert_eq!(thread_event.thread_info.thread_id.as_deref(), Some("thread_1"));
            match thread_event.thread_info.title {
                Some(qqbot_sdk::RichTextValue::Plain(ref title)) => assert_eq!(title, "Hello"),
                other => panic!("unexpected title: {other:?}"),
            }
            assert_eq!(
                thread_event
                    .thread_info
                    .extra
                    .get("custom_field")
                    .and_then(|v| v.as_i64()),
                Some(42)
            );
        }
        other => panic!("unexpected event data: {other:?}"),
    }
}
