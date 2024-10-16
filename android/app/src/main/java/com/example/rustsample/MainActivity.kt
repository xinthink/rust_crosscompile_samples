package com.example.rustsample

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import com.example.rustsample.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // Example of a call to a native method
        binding.sampleText.text = "Hello Rust ${MathLib.add(2000, 24)}"
    }

    companion object {
        // Used to load the 'rustsample' library on application startup.
        init {
            System.loadLibrary("hello")
        }
    }
}
