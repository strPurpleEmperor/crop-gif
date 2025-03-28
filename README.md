## 配合react-easy-crop使用
```shell
npm install react-easy-crop
npm install @zyss/crop-gif
```
```typescript jsx
import './App.css';
import { Button, Upload } from 'antd';
import Cropper, { Area } from 'react-easy-crop';
import { useLayoutEffect, useRef, useState } from 'react';
import init, { crop_gif } from '@zyss/crop-gif';

const App = () => {
  const [crop, setCrop] = useState({ x: 0, y: 0 });
  const [zoom, setZoom] = useState(1);
  const [yourImage, setImg] = useState<any>();
  const [file, setFile] = useState<File>();
  const area = useRef<Area>();
  useLayoutEffect(() => {
    init().then(() => {
      console.log('wasm init!');
    });
  }, []);
  const onCropComplete = (_croppedArea: Area, croppedAreaPixels: Area) => {
    area.current = {
      x: croppedAreaPixels.x,
      y: croppedAreaPixels.y,
      width: croppedAreaPixels.width,
      height: croppedAreaPixels.height,
    };
  };
  return (
    <div className="content">
      <Upload
        onChange={({ file }) => {
          setImg(URL.createObjectURL(file as any));
          setFile(file as any);
        }}
        beforeUpload={() => {
          return false;
        }}
      >
        <Button>上传</Button>
      </Upload>
      <div
        style={{
          width: 300,
          height: 300,
          position: 'absolute',
          top: 0,
          left: 0,
        }}
      >
        <Cropper
          image={yourImage}
          crop={crop}
          zoom={zoom}
          aspect={4 / 3}
          onCropChange={setCrop}
          onCropComplete={onCropComplete}
          onZoomChange={setZoom}
        />
      </div>
      <Button
        onClick={() => {
          const rustArea = area.current!;
          file?.arrayBuffer().then((gifBuffer) => {
            const uint8Array = new Uint8Array(gifBuffer);
            const cropGifU8Array = crop_gif(
              uint8Array,
              rustArea.width,
              rustArea.height,
              rustArea.x,
              rustArea.y,
            );
            const blob = new Blob([cropGifU8Array], { type: file!.type });
            const objectURL = URL.createObjectURL(blob);
            // 浏览器查看
            window.open(objectURL, '_blank');
          });
        }}
      >
        裁剪
      </Button>
    </div>
  );
};

export default App;

```