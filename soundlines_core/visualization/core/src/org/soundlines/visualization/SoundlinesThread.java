package org.soundlines.visualization;

import com.badlogic.gdx.utils.Json;
import org.soundlines.visualization.messages.ReceiveCells;
import org.soundlines.visualization.messages.ReceiveEntitiesMessage;
import org.soundlines.visualization.models.Cell;
import org.soundlines.visualization.models.Entity;

import java.util.ArrayList;
import java.util.concurrent.BlockingQueue;

public class SoundlinesThread implements Runnable {
	private BlockingQueue<ISoundlinesMessage> messageChannel;
	private BlockingQueue<ISoundlinesMessage> callbackChannel;

	SoundlinesThread(BlockingQueue<ISoundlinesMessage> messageChannel, BlockingQueue<ISoundlinesMessage> callbackChannel) {
		this.messageChannel = messageChannel;
		this.callbackChannel = callbackChannel;
	}

	@Override
	public void run() {
		SoundlinesCore.init();

		boolean running = true;
		while (running) {
			try {
				ISoundlinesMessage msg = messageChannel.take();

				switch (msg.GetType()) {
					case Stop: {
						running = false;
						break;
					}

					case FetchEntities: {
						String entitiesJson = SoundlinesCore.getEntities();
						Json json = new Json();
						json.setElementType(GetEntitiesResponse.class, "entities", Entity.class);
						json.setIgnoreUnknownFields(true);
						GetEntitiesResponse response = json.fromJson(GetEntitiesResponse.class, entitiesJson);
						callbackChannel.put(new ReceiveEntitiesMessage(response.entities));
						break;
					}

					case FetchCells: {
						String cellsJson = SoundlinesCore.getCells();
						Json json = new Json();
						json.setElementType(GetCellsResponse.class, "cells", Cell.class);
						json.setIgnoreUnknownFields(true);
						GetCellsResponse response = json.fromJson(GetCellsResponse.class, cellsJson);
						callbackChannel.put(new ReceiveCells(response.cells));
						break;
					}

					case ReceiveEntities:
					case ReceiveCells:
					default:
						break;
				}
			} catch (InterruptedException e) {
				e.printStackTrace();
			}
		}
	}

	static class GetCellsResponse {
		public ArrayList<Cell> cells;
	}

	static class GetEntitiesResponse {
		public ArrayList<Entity> entities;
	}
}
