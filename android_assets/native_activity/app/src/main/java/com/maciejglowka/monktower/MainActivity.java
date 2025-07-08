package com.maciejglowka.monktower;

// import androidx.appcompat.app.AppCompatActivity;

import android.app.NativeActivity;
import android.os.Bundle;
// import android.widget.TextView;

// import com.maciejglowka.monk_tower.databinding.ActivityMainBinding;

public class MainActivity extends NativeActivity {

    // Used to load the 'monktower' library on application startup.
    static {
        System.loadLibrary("main");
    }

//    private ActivityMainBinding binding;

//    @Override
//    protected void onCreate(Bundle savedInstanceState) {
//        super.onCreate(savedInstanceState);
//
//        binding = ActivityMainBinding.inflate(getLayoutInflater());
//        setContentView(binding.getRoot());
//
//        // Example of a call to a native method
//        TextView tv = binding.sampleText;
//        tv.setText(stringFromJNI());
//    }

}
