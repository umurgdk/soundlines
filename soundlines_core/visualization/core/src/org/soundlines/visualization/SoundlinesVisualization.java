package org.soundlines.visualization;

import com.badlogic.gdx.ApplicationAdapter;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.*;
import com.badlogic.gdx.graphics.g2d.Sprite;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;
import com.badlogic.gdx.math.Vector2;
import org.soundlines.visualization.messages.*;
import org.soundlines.visualization.models.Cell;
import org.soundlines.visualization.models.Entity;

import java.util.ArrayList;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingDeque;

public class SoundlinesVisualization extends ApplicationAdapter {
	private static final float WindowWidth = 2066;
	private static final float WindowHeight = 1600;
	private static final float ViewportWidth = WindowWidth / 2;
	private static final float ViewportHeight = WindowHeight / 2;

	private static final float MapGeoLeft = 126.989223f;
	private static final float MapGeoTop = 37.579291f;
	private static final float MapGeoRight = 127.015067f;
	private static final float MapGeoBottom = 37.563660f;

	private SpriteBatch batch;
	private ShapeRenderer shapeRenderer;
	private Sprite worldMap;
	private OrthographicCamera camera;
	private FPSLogger fpsLogger;
	private Vector2 dotPosition;

	private BlockingQueue<ISoundlinesMessage> messageChannel;
	private BlockingQueue<ISoundlinesMessage> callbackChannel;

	private ArrayList<Entity> entities;
	private ArrayList<Cell> cells;

	@Override
	public void create() {
		Gdx.graphics.setWindowedMode((int) WindowWidth, (int) WindowHeight);
		shapeRenderer = new ShapeRenderer();
		shapeRenderer.setAutoShapeType(true);

		batch = new SpriteBatch();
		camera = new OrthographicCamera(ViewportWidth, ViewportHeight);
		camera.setToOrtho(true, ViewportWidth, ViewportHeight);

		Texture worldImage = new Texture("worldmap.png");
		worldMap = new Sprite(worldImage);
		worldMap.flip(false, true);
		worldMap.setPosition(0, 0);

		fpsLogger = new FPSLogger();
		dotPosition = new Vector2(0, 0);

		entities = new ArrayList<>();
		cells = new ArrayList<>();

		messageChannel = new LinkedBlockingDeque<>();
		callbackChannel = new LinkedBlockingDeque<>();

		sendMessage(new FetchEntitiesMessage());
		sendMessage(new FetchCells());
		new Thread(new SoundlinesThread(messageChannel, callbackChannel)).start();
	}

	@Override
	public void render() {
		update();
		draw();

		fpsLogger.log();
	}

	private void draw() {
		Gdx.gl.glClearColor(0, 0, 0, 1);
		Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

		camera.setToOrtho(true, Gdx.graphics.getWidth() / 2.0f, Gdx.graphics.getHeight() / 2.0f);
		camera.update();
		batch.setProjectionMatrix(camera.combined);
		shapeRenderer.setProjectionMatrix(camera.combined);

		batch.begin();
		worldMap.draw(batch);
		batch.end();

		Gdx.gl.glEnable(GL20.GL_BLEND);
		Gdx.gl.glBlendFunc(GL20.GL_SRC_ALPHA, GL20.GL_ONE_MINUS_SRC_ALPHA);
		shapeRenderer.begin(ShapeRenderer.ShapeType.Filled);

		shapeRenderer.setColor(new Color(0x00000055));
		for (Cell cell : cells) {
			renderCell(cell);
		}

		shapeRenderer.setColor(new Color(0x88ccff88));
		for (Entity entity : entities.stream().limit(20000).toArray(Entity[]::new)) {
			renderEntity(entity);
		}

		shapeRenderer.end();
	}

	private void renderEntity(Entity entity) {
		shapeRenderer.set(ShapeRenderer.ShapeType.Filled);
		Vector2 pixelPos = geoToPixel(entity.point[0], entity.point[1]);
		shapeRenderer.circle(pixelPos.x, pixelPos.y, 2);
	}

	private void renderCell(Cell cell) {
		Vector2 topLeft = geoToPixel(cell.geom[0][0], cell.geom[0][1]);
		Vector2 bottomLeft = geoToPixel(cell.geom[1][0], cell.geom[1][1]);
		Vector2 topRight = geoToPixel(cell.geom[3][0], cell.geom[3][1]);
		Vector2 bottomRight = geoToPixel(cell.geom[2][0], cell.geom[2][1]);

		shapeRenderer.set(ShapeRenderer.ShapeType.Filled);
		shapeRenderer.rect(topLeft.x, topLeft.y, topRight.x - topLeft.x, bottomLeft.y - topLeft.y);
		Gdx.gl.glLineWidth(4);
		shapeRenderer.set(ShapeRenderer.ShapeType.Line);
		shapeRenderer.rect(topLeft.x, topLeft.y, topRight.x - topLeft.x, bottomLeft.y - topLeft.y);
	}

	private void update() {
		handleSoundlinesCallbacks();

		dotPosition.x = Gdx.input.getX() / 2;
		dotPosition.y = Gdx.input.getY() / 2;
	}

	private Vector2 geoToPixel(float geo_x, float geo_y) {
		float x = ViewportWidth * (geo_x - MapGeoLeft) / (MapGeoRight - MapGeoLeft);
		float y = ViewportHeight - ViewportHeight * (geo_y - MapGeoBottom) / (MapGeoTop - MapGeoBottom);
		return new Vector2(x, y);
	}

	private void handleSoundlinesCallbacks() {
		ISoundlinesMessage msg = callbackChannel.poll();

		if (msg == null) {
			return;
		}

		switch (msg.GetType()) {
			case ReceiveEntities:
				this.entities = ((ReceiveEntitiesMessage) msg).entities;
				break;

			case ReceiveCells:
				this.cells = ((ReceiveCells) msg).cells;
				break;

			default:
				break;
		}
	}

	private void sendMessage(ISoundlinesMessage message) {
		try {
			messageChannel.put(message);
		} catch (InterruptedException e) {
			System.out.println("Failed to get message from messageChannel");
			e.printStackTrace();
		}
	}

	@Override
	public void dispose() {
		sendMessage(new StopMessage());
		batch.dispose();
		shapeRenderer.dispose();
	}
}
