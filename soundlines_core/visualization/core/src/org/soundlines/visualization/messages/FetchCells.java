package org.soundlines.visualization.messages;

import org.soundlines.visualization.ISoundlinesMessage;
import org.soundlines.visualization.MessageType;

public class FetchCells implements ISoundlinesMessage {
	@Override
	public MessageType GetType() {
		return MessageType.FetchCells;
	}
}
