# Creating a Demo GIF for FlowNet

This guide walks through recording a professional demo GIF and preparing it for the `assets/demo.gif` placeholder.

---

## Step 1 — Choose a recorder

| OS | Recommended tool | Why |
|----|-----------------|-----|
| Windows | [ScreenToGif](https://www.screentogif.com/) | Free, open-source, records directly to GIF, has built-in editor for blurring/cropping |
| Windows / macOS / Linux | OBS Studio + FFmpeg | Record a high-quality MP4, then convert to GIF with FFmpeg for more control |
| macOS | GIPHY Capture (App Store) | Simple, free, records directly to GIF |

**Recommendation:** ScreenToGif (Windows) — it handles everything in one tool.

---

## Step 2 — Prepare the capture environment

1. Build FlowNet in dev mode:
   ```bash
   cargo tauri dev
   ```
2. Generate **safe, non‑sensitive traffic** so no real IPs appear:
   - Visit `http://example.com` or `http://httpbin.org/get` in a browser
   - Ping `localhost` or `127.0.0.1`
   - Use `curl http://localhost:8080` if you have a local server
   - **Avoid** visiting banking sites, internal corporate tools, or any real services
3. Close other apps that generate background network noise.

---

## Step 3 — Record the screen

### Using ScreenToGif

1. Launch ScreenToGif → **Recorder** → **Window**.
2. Resize the capture rectangle to fit the FlowNet window (~1280×720).
3. Click **Record** (F7).
4. In FlowNet:
   - Select a loopback interface (e.g. `Adapter for loopback traffic capture`)
   - Click **Start**
   - Wait for packets to appear (generate traffic via `ping 127.0.0.1 -t`)
   - Click a packet to show the detail panel
   - Click **Stop**
5. Press **F8** to stop recording.

### Using OBS + FFmpeg

1. OBS: add **Window Capture** source → select FlowNet window.
2. Start recording (MKV or MP4, 30 FPS).
3. Perform the same actions as above.
4. Stop recording, then convert to GIF:
   ```bash
   ffmpeg -i recording.mp4 -vf "fps=12,scale=854:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" demo.gif
   ```

---

## Step 4 — Blur confidential data

Before saving, blur any visible IP addresses, MAC addresses, or hostnames.

### ScreenToGif (built‑in)

1. Go to **Edit** → **Freehand blur** (or **Rectangle blur** for IP columns).
2. Select the area covering the IP addresses in the table.
3. Apply to **All frames** (not just the current frame).
4. Adjust blur strength so the text is unreadable but the layout is still visible.

### FFmpeg (post‑processing)

```bash
ffmpeg -i demo.gif -vf "drawbox=x=100:y=50:w=200:h=20:color=black@0.6:t=fill" blurred.gif
```

Adjust `x`, `y`, `w`, `h` to cover the IP columns. Run this on every Nth frame if needed, or use a more complex filter.

### Manual alternative

Use any video editor (DaVinci Resolve, Shotcut) to place a blur overlay on the timeline, then export and convert to GIF.

---

## Step 5 — Optimize the GIF

| Tool | Command |
|------|---------|
| [gifsicle](https://www.lcdf.org/gifsicle/) | `gifsicle -O3 --lossy=80 -o optimized.gif demo.gif` |
| [ezgif.com](https://ezgif.com/optimize) | Browser-based, drag-and-drop |

Optimization shrinks the file to <5 MB while keeping quality acceptable.

---

## Step 6 — Place in the repo

```bash
cp optimized.gif assets/demo.gif
git add assets/demo.gif
```

Then remove the `*Screenshot coming soon…*` note from README.md.

---

## Security checklist before uploading

- [ ] No real public IP addresses visible
- [ ] No private / internal IPs visible (unless obviously localhost)
- [ ] No MAC addresses readable
- [ ] No usernames, hostnames, or domain names from internal networks
- [ ] Only loopback (`127.0.0.1`) or example (`192.0.2.x`) addresses remain visible
