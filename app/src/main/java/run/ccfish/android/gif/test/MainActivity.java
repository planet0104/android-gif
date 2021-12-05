package run.ccfish.android.gif.test;

import android.os.Bundle;
import android.util.Log;

import androidx.appcompat.app.AppCompatActivity;

import java.io.IOException;
import run.ccfish.android.gif.Frame;
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
                    Frame[] frames = Gif.decodeUrl("https://www.ccfish.run/head_anim.gif");
                    Log.i("Main", "frames="+frames.length);
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }).start();
    }
}