package run.ccfish.android.gif.test;

import android.graphics.Bitmap;
import android.graphics.drawable.AnimationDrawable;
import android.graphics.drawable.BitmapDrawable;
import android.graphics.drawable.Drawable;
import android.os.Bundle;
import android.util.Log;

import androidx.appcompat.app.AppCompatActivity;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;

import run.ccfish.android.gif.Gif;

public class MainActivity extends AppCompatActivity {
    static final String TAG = MainActivity.class.getSimpleName();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        File gif = new File(getFilesDir(), "test.gif");
        testDecode(gif, R.id.gif1, 10);
        testEncode();
    }

    private void testEncode(){
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    File gif = new File(getFilesDir(), "out.gif");
                    int width = 274;
                    int height = 274;
                    final String filePath = gif.getAbsolutePath();
                    Gif.createEncoder(filePath, true, width, height, 20);
                    File frameDir = new File(getFilesDir(), "gif");
                    frameDir.mkdirs();
                    for(int i=1; i<=97; i++){
                        File frame = new File(frameDir, i+".png");
                        if(!frame.exists()){
                            InputStream in = getAssets().open("gif/"+i+".png");
                            FileUtils.inputStreamToFile(in, frame);
                        }
                        Gif.appendFileToEncoder(filePath, frame.getAbsolutePath());
                    }
                    Gif.closeEncoder(filePath);
                    runOnUiThread(new Runnable() {
                        @Override
                        public void run() {
                            testDecode(gif, R.id.gif2, 20);
                        }
                    });
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }
        }).start();
    }

    private void testDecode(final File gif, int resId, int fps){
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    if(!gif.exists()){
                        InputStream in = getAssets().open("test.gif");
                        FileUtils.inputStreamToFile(in, gif);
                    }
                    List<Bitmap> bitmapList = new ArrayList<>();
                    boolean success = Gif.decodeFile(gif, new Gif.OnFrameDataCallback() {
                        @Override
                        public void onFrameData(Bitmap bitmap) {
                            bitmapList.add(bitmap);
                        }
                    });
                    Log.i(TAG, "解码success="+success);
                    Log.i(TAG, "assets路径:"+getFilesDir().getAbsolutePath());

                    // 创建一个AnimationDrawable对象
                    AnimationDrawable animationDrawable = new AnimationDrawable();

                    // 遍历Bitmap数组，把每个Bitmap对象转换成Drawable对象，然后添加到动画中
                    for (Bitmap bitmap : bitmapList) {
                        // 把Bitmap对象转换成Drawable对象
                        Drawable drawable = new BitmapDrawable(getResources(), bitmap);
                        // 把Drawable对象添加到动画中，设置每一帧的持续时间为100毫秒
                        animationDrawable.addFrame(drawable, 1000/fps);
                    }

                    // 设置动画是否只播放一次，false表示循环播放
                    animationDrawable.setOneShot(false);

                    runOnUiThread(new Runnable() {
                        @Override
                        public void run() {
                            findViewById(resId).setBackground(animationDrawable);
                            animationDrawable.start();
                        }
                    });
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }).start();
    }
}