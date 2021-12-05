package run.ccfish.android.gif;

import android.graphics.Bitmap;

public class Frame {
    /** Frame delay in units of 10 ms. */
    public final int delay;
    /**
     * Disposal method.
     *     // StreamingDecoder is not required to take any action.
     *     Any = 0,
     *     // Do not dispose.
     *     Keep = 1,
     *     // Restore to background color.
     *     Background = 2,
     *     // Restore to previous.
     *     Previous = 3,
     */
    public final int dispose;
    /** Transparent index (if available). */
    public final int transparent;
    /** True if the frame needs user input to be displayed. */
    public final boolean needs_user_input;
    /** Offset from the top border of the canvas. */
    public final int top;
    /** Offset from the left border of the canvas. */
    public final int left;
    /** Width of the frame. */
    public final int width;
    /** Height of the frame. */
    public final int height;
    /** True if the image is interlaced. */
    public final boolean interlaced;
    /** image data */
    public final Bitmap image;

    /**
     * Frame
     * @param delay
     * @param dispose
     * @param transparent
     * @param needs_user_input
     * @param top
     * @param left
     * @param width
     * @param height
     * @param interlaced
     * @param image
     */
    public Frame(int delay, int dispose, int transparent, boolean needs_user_input, int top, int left, int width, int height, boolean interlaced, Bitmap image) {
        this.delay = delay;
        this.dispose = dispose;
        this.transparent = transparent;
        this.needs_user_input = needs_user_input;
        this.top = top;
        this.left = left;
        this.width = width;
        this.height = height;
        this.interlaced = interlaced;
        this.image = image;
    }

    public int getDelayMs() {
        return delay * 10;
    }
}
