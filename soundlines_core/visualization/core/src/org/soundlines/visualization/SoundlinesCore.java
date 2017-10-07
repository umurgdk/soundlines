package org.soundlines.visualization;

class SoundlinesCore {
	static native void init();
	static native String getEntities();
	static native String getCells();

	static {
		System.loadLibrary("soundlines_core");
	}
}
