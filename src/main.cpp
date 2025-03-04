#include <SDL2/SDL_events.h>
#include <SDL2/SDL_render.h>
#include <SDL2/SDL.h>
#include <SDL2/SDL_timer.h>
#include <cmath>
#include <cstdio>
#include <thread>

#include "iset.h"
#include "cpu.h"

void display_tick(SDL_Renderer *renderer, Cpu *cpu) {
    SDL_Event event;
    // while (true) {
        while (SDL_PollEvent(&event)) {
            switch (event.type) {
                case SDL_QUIT:
                    SDL_Quit();
                    exit(0);
            }
        }

        for (unsigned int i = 0; i < 854 * 480 * 4; i += 4) {
            byte r = cpu->memory[0xa0000 + i];
            byte g = cpu->memory[0xa0000 + i + 1];
            byte b = cpu->memory[0xa0000 + i + 2];
            byte a = 255;
            if (!(r || g || b)) continue;
            // byte a = cpu.memory[0xa0000 + i + 3];
            unsigned int pixel = i / 4;
            SDL_SetRenderDrawColor(renderer, r, g, b, a);
            SDL_RenderDrawPoint(renderer, pixel % 854, pixel / 854);

            // printf("%0*lx\n", (unsigned int)(sizeof(arch) * 2), cpu.rip);
        }

        // SDL_Delay(1000.0f / 60.0f);

        SDL_RenderPresent(renderer);
    // }
}
void process_tick(Cpu *cpu) {
    // while (true) {
        next_instruction(cpu);
    // }
}

int main(void) {
    Program program = {
        { MovRegImm, 0b00000001, }, split_number(0xa0000),
        { MovRegReg, 0b01010001, },
        { AddRegImm, 0b00000001, }, split_number(4),
        { AddRegImm, 0b00000010, }, split_number(1),
        { MovRegImm, 0b00000111, }, split_number(10),
    };

    Cpu cpu = initialize_cpu();

    cpu.rbp = load_program(&cpu, program);
    cpu.rsp = cpu.rbp;
    // execute_program(&cpu);

    print_stack_context(&cpu, 0, (int[2]){ 0, 127 });
    // print_cpu_state(&cpu);

    SDL_Window *window;
    SDL_Renderer *renderer;
    SDL_CreateWindowAndRenderer(854, 480, 0, &window, &renderer);

    // std::thread process_thread(process_tick, &cpu);
    // std::thread display_thread(display_tick, renderer, &cpu);

    while (true) {
        process_tick(&cpu);
        display_tick(renderer, &cpu);
    }

    return 0;
}
