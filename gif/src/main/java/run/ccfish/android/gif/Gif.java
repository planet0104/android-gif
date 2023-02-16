package run.ccfish.android.gif;

import android.graphics.Bitmap;

import com.sun.jna.Callback;
import com.sun.jna.Library;
import com.sun.jna.Memory;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;

public class Gif {
    static final String TAG = Gif.class.getSimpleName();

    public interface DecodeFrameCallback extends Callback {
        void callback(Pointer pointer, int width, int height);
    }

    public interface OnFrameDataCallback{
        void onFrameData(Bitmap bitmap);
    }

    interface GifSys extends Library {
        GifSys INSTANCE = (GifSys) Native.load("android_gif", GifSys.class);
        /**
         * 解码Gif
         * @param file_data 文件的字节数组
         * @param size 字节数组长度
         * @param overlay 是否覆盖绘制
         * @param callback 解码后的回调函数
         * @return 0 成功 1失败
         */
        int decode_bytes(Pointer file_data, int size, boolean overlay, DecodeFrameCallback callback);

        /**
         * 解码Gif
         * @param file gif文件名
         * @param size 字节数组长度
         * @param overlay 是否覆盖绘制
         * @param callback 解码后的回调函数
         * @return 0 成功 1失败
         */
        int decode_file(String file, boolean overlay, DecodeFrameCallback callback);
        int create_encoder(String file, boolean repeat, int width, int height, int fps);
        int append_file_to_encoder(String file, String image_file);
        int append_bitmap_to_encoder(String file, int[] colors, int size);
        int close_decoder(String file);
    }

    /**
     * Decode Image from InputStream
     * @param input InputStream
     * @return Frame[]
     */
    public static boolean decodeStream(InputStream input, boolean overlay, OnFrameDataCallback decodeFrameCallback) throws IOException {
        ByteArrayOutputStream output = new ByteArrayOutputStream();
        byte[] buffer = new byte[4096];
        int n = 0;
        while (-1 != (n = input.read(buffer))) {
            output.write(buffer, 0, n);
        }
        byte[] bytes = output.toByteArray();
        Pointer pointer = new Memory(bytes.length);
        pointer.write(0, bytes, 0, bytes.length);
        return GifSys.INSTANCE.decode_bytes(pointer, bytes.length, overlay, (data, width, height) -> {
            int[] colors = data.getIntArray(0, width*height);
            Bitmap bitmap = Bitmap.createBitmap(colors, width, height, Bitmap.Config.ARGB_8888);
            decodeFrameCallback.onFrameData(bitmap);
        }) == 0;
    }

    /**
     * Decode Image from URL
     * @param url URL
     * @return Frame[]
     * @throws IOException Exception
     */
    public static boolean decodeUrl(String url, OnFrameDataCallback decodeFrameCallback) throws IOException {
        return decodeStream(new URL(url).openStream(), true, decodeFrameCallback);
    }

    /**
     * Decode Image from URL
     * @param url
     * @param overlay
     * @param decodeFrameCallback
     * @return
     * @throws IOException
     */
    public static boolean decodeUrl(String url, boolean overlay, OnFrameDataCallback decodeFrameCallback) throws IOException {
        return decodeStream(new URL(url).openStream(), overlay, decodeFrameCallback);
    }

    /**
     *
     * @param file
     * @param decodeFrameCallback
     * @return
     */
    public static boolean decodeFile(File file, OnFrameDataCallback decodeFrameCallback) {
        return decodeFile(file, true, decodeFrameCallback);
    }

    /**
     * Decode Image from File
     * @param file File
     * @return Frame[]
     */
    public static boolean decodeFile(File file, boolean overlay, OnFrameDataCallback decodeFrameCallback) {
        return GifSys.INSTANCE.decode_file(file.getAbsolutePath(),overlay, (data, width, height) -> {
            int[] colors = data.getIntArray(0, width*height);
            Bitmap bitmap = Bitmap.createBitmap(colors, width, height, Bitmap.Config.ARGB_8888);
            decodeFrameCallback.onFrameData(bitmap);
        }) == 0;
    }

    public static int createEncoder(String file, boolean repeat, int width, int height, int fps){
        return GifSys.INSTANCE.create_encoder(file, repeat, width, height, fps);
    }
    public static int appendFileToEncoder(String file, String image_file){
        return GifSys.INSTANCE.append_file_to_encoder(file, image_file);
    }
    public static int appendBitmapToEncoder(String file, Bitmap bitmap){
        int w = bitmap.getWidth();
        int h = bitmap.getHeight();
        int[] pixels = new int[w * h];
        bitmap.getPixels(pixels, 0, w, 0, 0, w, h);
        return GifSys.INSTANCE.append_bitmap_to_encoder(file, pixels, w*h);
    }
    public static int closeEncoder(String file){
        return GifSys.INSTANCE.close_decoder(file);
    }
}
