package org.soundlines.visualization.messages;

import org.soundlines.visualization.ISoundlinesMessage;
import org.soundlines.visualization.MessageType;
import org.soundlines.visualization.models.Cell;

import java.util.ArrayList;

public class ReceiveCells implements ISoundlinesMessage {
	public ArrayList<Cell> cells;

	public ReceiveCells(ArrayList<Cell> cells) {
		this.cells = cells;
	}

	@Override
	public MessageType GetType() {
		return MessageType.ReceiveCells;
	}
}
