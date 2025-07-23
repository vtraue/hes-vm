package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

class BytecodeWriter {
    ByteArrayOutputStream os;

    BytecodeWriter() {
        os = new ByteArrayOutputStream();
    }

    void reset(){
        os.reset();
    }
    byte [] toByteArray() { return os.toByteArray();}

    ByteArrayOutputStream getOutputStream() { return os; }

    void writeByte(byte b) throws IOException {
        byte[] code = { b };
        os.write(code);
    }

    void writeBytes(byte [] b) throws IOException {
        os.write(b);
    }

    void writeBytes(List<Integer> al) throws IOException{
        for (Integer e : al) {
            byte[] byteId = { (byte) e.intValue() };
            os.write(byteId);
        }
    }

    void writeU32(int num) throws IOException {
        writeBytes(encodeU32ToLeb128(num));
    }

    void writeI32(int num) throws IOException{
        writeBytes(encodeI32ToLeb128(num));
    }

    void writeI64(int num) throws IOException{
        // TODO
    }

    void writeF32(double num) throws IOException {
        // TODO
    }

    void writeF64(double num) throws IOException {
        // TODO
    }

    void writeOpcode(InstructionOpCode opcode) throws IOException {
        writeByte((byte) opcode.code);
    }

    void writeWithSize(ByteArrayOutputStream input) throws IOException {
        writeU32(input.size());
        os.write(input.toByteArray());
    }

    private static ArrayList<Integer> encodeU32ToLeb128(int value) {
        value |= 0;
        ArrayList<Integer> result = new ArrayList<>();
        while (true) {
            int byte_ = value & 0x7f;
            value >>= 7;
            if (value == 0) {
                result.add(byte_);
                return result;
            }
            result.add(byte_ | 0x80);
        }
    }

    private static ArrayList<Integer> encodeI32ToLeb128(int value) {
        value |= 0;
        ArrayList<Integer> result = new ArrayList<>();
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
