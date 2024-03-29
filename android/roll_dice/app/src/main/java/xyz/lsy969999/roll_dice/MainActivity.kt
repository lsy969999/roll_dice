package xyz.lsy969999.roll_dice

import android.os.Bundle
import android.os.PersistableBundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import xyz.lsy969999.roll_dice.ui.theme.Roll_diceTheme
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            Roll_diceTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    SurfaceCard()
                }
            }
        }
    }

    override fun onSaveInstanceState(outState: Bundle, outPersistentState: PersistableBundle) {
        super.onSaveInstanceState(outState, outPersistentState)
    }

    override fun onRestoreInstanceState(
        savedInstanceState: Bundle?,
        persistentState: PersistableBundle?
    ) {
        super.onRestoreInstanceState(savedInstanceState, persistentState)
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    Roll_diceTheme {
        Greeting("Android")
    }
}

var surfaceView: BevySurfaceView? = null

@Composable
fun SurfaceCard() {
    val screenWidth = LocalConfiguration.current.screenWidthDp
    Column(modifier = Modifier.fillMaxSize()) {
//        Row(
//            verticalAlignment = Alignment.CenterVertically,
//            horizontalArrangement = Arrangement.Center,
//            modifier = Modifier
//                .height(44.dp)
//                .padding(horizontal = 0.dp, vertical = 7.dp)
//                .fillMaxWidth()
//        ) {
//            Text(text = "Roll Dice", fontSize = 20.sp, fontWeight = FontWeight.Bold)
//        }
//        Spacer(modifier = Modifier.height(8.dp))
        AndroidView(
            factory = {ctx ->
                val sv = BevySurfaceView(context = ctx)
//                surfaceView = sv
                sv
            },
//            modifier = Modifier
//                .fillMaxWidth()
//                .height((screenWidth.toFloat() * 1.6).dp)
        )
    }
}