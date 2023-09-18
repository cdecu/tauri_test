import { ChangeDetectorRef, Component, computed, effect, OnDestroy, OnInit, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatCardModule } from '@angular/material/card';
import { MatSliderModule } from '@angular/material/slider';
import { NzMarks, NzSliderModule } from 'ng-zorro-antd/slider';
import { createTauRPCProxy } from '../lib/bindings';
import { NgComposModule } from '@gt2/ui/ng-compos';
import { calculatePressure } from '@gt2/ts/utils';
import { UnlistenFn } from '@tauri-apps/api/event';
import {MatIconModule} from "@angular/material/icon";

@Component({
  standalone: true,
  imports: [FormsModule, MatCardModule, MatSliderModule, NzSliderModule, NgComposModule, MatIconModule],
  selector: 'gt-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit, OnDestroy {
  title = 'Permeability';
  logs = 'Starting...';
  ev: UnlistenFn = () => void 0;
  flowrate_marks: NzMarks = {
    0: '0[l/min]',
    100: { label: '<strong>100[l/min]</strong>', style: { color: 'red' } },
  };

  flowrate_value = signal(25);
  flow_values = computed<{ flowrate: number; pressure: number }>(() => {
    const flowrate = this.flowrate_value();
    const pressure = calculatePressure(flowrate);
    return { flowrate, pressure };
  });

  constructor(private changeDetector: ChangeDetectorRef) {
    effect(async () => {
      const flowrate = this.flowrate_value();
      const pressure = calculatePressure(flowrate);
      const taurpc = await createTauRPCProxy();
      taurpc.set_permeability(flowrate, pressure);
    });
  }

  async ngOnInit() {
    // invoke retourne une promesse
    const taurpc = await createTauRPCProxy();
    // taurpc.
    this.ev = taurpc.events.ev.on((log) => {
      this.logs = '' + log + '\n' + this.logs;
      this.changeDetector.detectChanges();
    });
    // taurpc.test_error('test').catch((err) => {
    //   console.error(err);
    // });
  }

  ngOnDestroy(): void {
    this.ev();
  }

  setFlow($event: number) {
    this.flowrate_value.set($event);
    this.changeDetector.detectChanges();
  }

  clearLogs() {
    this.logs = '';
    this.changeDetector.detectChanges();
  }
}
