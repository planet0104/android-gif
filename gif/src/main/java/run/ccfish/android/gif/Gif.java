package run.ccfish.android.gif;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;

public class Gif {
    static {
        System.loadLibrary("android_gif");
    }

    public static native Frame[] decode(byte[] data);

    /**
     * Decode Image from InputStream
     * @param input InputStream
     * @return Frame[]
     */
    public static Frame[] decodeStream(InputStream input) throws IOException {
        ByteArrayOutputStream output = new ByteArrayOutputStream();
        byte[] buffer = new byte[4096];
        int n = 0;
        while (-1 != (n = input.read(buffer))) {
            output.write(buffer, 0, n);
        }
        return decode(output.toByteArray());
    }

    /**
     * Decode Image from URL
     * @param url URL
     * @return Frame[]
     * @throws IOException Exception
     */
    public static Frame[] decodeUrl(String url) throws IOException {
        return decodeStream(new URL(url).openStream());
    }

    /**
     * Decode Image from File
     * @param file File
     * @return Frame[]
     */
    public static Frame[] decodeFile(File file) throws IOException {
        return decodeStream(new FileInputStream(file));
    }
}
