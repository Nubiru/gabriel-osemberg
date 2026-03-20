# {STREAM_NAME} — Outbox (messages to other streams)

<!--
Format for outgoing messages:

## {DATE} — {THIS_STREAM} -> {TARGET_STREAM} — Priority: {HIGH/MEDIUM/LOW}

**Subject**: {SUBJECT}
**Status**: {PENDING/DELIVERED/RESOLVED}

{MESSAGE_BODY}

---

IMPORTANT: When writing to outbox, ALSO write the same message
directly to the target stream's inbox.md at:
  .context/streams/{TARGET_STREAM}/inbox.md
-->

No messages sent.
