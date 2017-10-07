package org.soundlines.visualization;

class SoundlinesCore {
	static native void init();
	static native String getEntities();

	static {
		System.loadLibrary("soundlines_core");
	}

	public static void main(String[] args) {
		SoundlinesCore.init();
		System.out.println(getEntities());
	}
}
