# android-gif


[![](https://jitpack.io/v/planet0104/android-gif.svg)](https://jitpack.io/#planet0104/android-gif)


Android Gif Decoder


```java
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
```
