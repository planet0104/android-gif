# android-gif


[![](https://jitpack.io/v/planet0104/android-gif.svg)](https://jitpack.io/#planet0104/android-gif)


Android Gif Decoder


build.gradle

```gradle
dependencies {
    implementation 'com.github.planet0104:android-gif:1.0.3release2'
}
```

settings.gradle

```gradle
dependencyResolutionManagement {
    //...
    repositories {
        //...
        maven { url 'https://jitpack.io' }
    }
}
```

MainActivity

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
