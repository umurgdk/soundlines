package org.soundlines.visualization.messages;

import org.soundlines.visualization.ISoundlinesMessage;
import org.soundlines.visualization.MessageType;
import org.soundlines.visualization.models.Entity;

import java.util.ArrayList;

public class ReceiveEntitiesMessage implements ISoundlinesMessage {
	public ArrayList<Entity> entities;

	public ReceiveEntitiesMessage(ArrayList<Entity> entities) {
		this.entities = entities;
	}

	@Override
	public MessageType GetType() {
		return MessageType.ReceiveEntities;
	}
}
