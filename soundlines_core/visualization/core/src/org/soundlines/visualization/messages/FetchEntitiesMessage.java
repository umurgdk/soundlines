package org.soundlines.visualization.messages;

import org.soundlines.visualization.ISoundlinesMessage;
import org.soundlines.visualization.MessageType;

public class FetchEntitiesMessage implements ISoundlinesMessage {

	@Override
	public MessageType GetType() {
		return MessageType.FetchEntities;
	}
}
