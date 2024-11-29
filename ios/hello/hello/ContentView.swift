//
//  ContentView.swift
//  hello
//
//  Created by Yingxin Wu on 2024/11/28.
//

import SwiftUI

struct ContentView: View {
    @State var message = rust_greeting("Rust world")!

    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text(String(cString: message))
        }
        .padding()
    }

    func onDisappear() {
      rust_greeting_free(UnsafeMutablePointer(mutating: message))
    }
}

#Preview {
    ContentView()
}
