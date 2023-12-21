//
//  ContentView.swift
//  Memory
//
//  Created by Tanner on 10/22/23.
//

import SwiftUI

struct ContentView: View {
   
    var body: some View {
        VStack(alignment: .leading) {
            Text("Coffee & Tea Collective").font(.title)
            HStack() {
                Text("Tanner").font(.subheadline)
                Spacer()
                Text("Developer").font(.subheadline)
            }
            MapView().frame(height: 200)
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
