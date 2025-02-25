import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;

public class WasmBuilder {

	ByteArrayOutputStream out = new ByteArrayOutputStream();

	public void addBinaryMagic() throws IOException {
		byte[] wasmBinaryMagic = { 0x0, 'a', 's', 'm' };
		out.write(wasmBinaryMagic);
	}

	public void addBinaryVersion() throws IOException {
		byte[] wasmBinaryVersion = { 0x01, 0x00, 0x00, 0x00 };
		out.write(wasmBinaryVersion);

	}

	public static ArrayList<Integer> encodeI32ToLeb128(int value) {
		value |= 0;
		ArrayList<Integer> result = new ArrayList<Integer>();
		while (true) {

			int byte_ = value & 0x7f;
			value >>= 7;
			if ((value == 0 && (byte_ & 0x40) == 0) || (value == -1 && (byte_ & 0x40) != 0)) {
				result.add(byte_);
				return result;
			}
			result.add(byte_ | 0x80);
		}

	}

}
