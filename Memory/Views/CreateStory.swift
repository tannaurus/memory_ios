//
//  CreateStory.swift
//  Memory
//
//  Created by Tanner on 11/25/23.
//

import SwiftUI

struct CreateStory: View {
    var prompts: [Prompt]
    
    struct Card: View {
        var prompt: Prompt
        
        var body: some View {
            VStack(alignment: .leading) {
                Text(prompt.name).font(.system(size: 24)).fontWeight(/*@START_MENU_TOKEN@*/.bold/*@END_MENU_TOKEN@*/)
                Text(prompt.description)
            }
            .frame(minHeight: /*@START_MENU_TOKEN@*/100/*@END_MENU_TOKEN@*/)
        }
    }
    
    var body: some View {
        NavigationSplitView {
            List(prompts) { prompt in
                NavigationLink {
                    Text(prompt.name)
                } label: {
                    Card(prompt: prompt)
                }
            }
            .listStyle(.plain)
            .navigationTitle("Create")
        } detail: {
            Text("Select a prompt")
        }
    }
}

#Preview {
    CreateStory(prompts: mock_prompts)
}
