import os
import sys
import numpy as np
import pyqtgraph as pg
import pyqtgraph.exporters


class Visualiser:
    def __init__(self, dnames, lengths, export):
        # create graphic window and figure
        w = 600
        h = 600
        win = pg.GraphicsLayoutWidget(parent=None, show=True, size=(w, h), title="window")
        win.setWindowTitle("Collision animation")
        self.win = win
        self.export = export
        self.plt = win.addPlot()
        xmin = 0. * lengths[0]
        xmax = 1. * lengths[0]
        ymin = 0. * lengths[1]
        ymax = 1. * lengths[1]
        margin = 0.01
        self.plt.setXRange(min=xmin-margin*lengths[0], max=xmax+margin*lengths[0])
        self.plt.setYRange(min=ymin-margin*lengths[1], max=ymax+margin*lengths[1])
        self.plt.plot([xmin, xmin], [ymin, ymax], pen="#FFFFFF")
        self.plt.plot([xmax, xmax], [ymin, ymax], pen="#FFFFFF")
        self.plt.plot([xmin, xmax], [ymin, ymin], pen="#FFFFFF")
        self.plt.plot([xmin, xmax], [ymax, ymax], pen="#FFFFFF")
        self.plt.enableAutoRange("xy", False)
        self.plt.setAspectLocked(lock=True, ratio=1)
        self.plt.showAxes(selection=False, showValues=False, size=False)
        self.scatter = self.plt.plot(
                x=[],
                y=[],
                pen=None, # don't draw lines between points
                symbol="o", # circle
                symbolPen=None, # no additional draw for each dots
                symbolSize=[], # diameter
                pxMode=False,
                symbolBrush="#FF0000",
        )
        timer = pg.QtCore.QTimer()
        self.i = 0
        self.dnames = dnames
        timer.timeout.connect(self.update)
        timer.start(1) # refresh display per X ms
        if (sys.flags.interactive != 1) or not hasattr(pg.QtCore, "PYQT_VERSION"):
            pg.QtGui.QGuiApplication.instance().exec_()

    def update(self, *args):
        rs = np.load(dnames[self.i] + "/radii.npy")
        xs = np.load(dnames[self.i] + "/positions_0.npy")
        ys = np.load(dnames[self.i] + "/positions_1.npy")
        self.plt.setTitle("{:d} / {:d}".format(self.i, len(dnames)))
        self.scatter.setData(x=xs, y=ys, symbolSize=2.*rs)
        pg.QtGui.QGuiApplication.processEvents()
        if self.export:
            exporter = pg.exporters.ImageExporter(self.win.scene())
            exporter.export("img/img{:05d}.png".format(self.i))
        self.i += 1
        if self.i >= len(self.dnames):
            sys.exit()


if __name__ == "__main__":
    lengths = np.load("input/lengths.npy")
    root = "output/save"
    dnames = [f"{root}/{dname}" for dname in os.listdir(root) if dname.startswith("iter")]
    dnames = sorted(dnames)
    pg.setConfigOptions(antialias=False)
    Visualiser(
            dnames=dnames,
            lengths=lengths,
            export=False,
    )

