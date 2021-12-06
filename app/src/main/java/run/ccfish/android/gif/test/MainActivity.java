package run.ccfish.android.gif.test;

import android.os.Bundle;
import android.util.Log;

import androidx.appcompat.app.AppCompatActivity;

import java.io.IOException;

import run.ccfish.android.gif.DecodeGif;
import run.ccfish.android.gif.Gif;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    DecodeGif decodeGif = Gif.decodeUrl("https://www.ccfish.run/girl.gif");
                    Log.i("Main", "frames="+decodeGif.frames.length);
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }).start();
    }
}