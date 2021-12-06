package run.ccfish.android.gif;

/**
 * 解析好的GIF
 */
public class DecodeGif {
    public final int width;
    public final int height;
    public final Frame[] frames;

    public DecodeGif(int width, int height, Frame[] frames) {
        this.width = width;
        this.height = height;
        this.frames = frames;
    }
}
