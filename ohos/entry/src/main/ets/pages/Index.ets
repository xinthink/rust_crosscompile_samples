import { hilog } from '@kit.PerformanceAnalysisKit';
// import testNapi from 'libentry.so';
import testNapi from 'libhello.so';

@Entry
@Component
struct Index {
  @State message: string = 'Hello World';
  @State result: number = 0;

  build() {
    Row() {
      Column() {
        Text(`${this.message} ${this.result}`)
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            this.result = testNapi.add(this.result, 2);
            hilog.info(0x0000, 'testTag', 'Test NAPI 2 + 3 = %{public}d', this.result);
          })
      }
      .width('100%')
    }
    .height('100%')
  }
}
