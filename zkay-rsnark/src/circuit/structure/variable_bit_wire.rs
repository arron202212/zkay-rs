

public class VariableBitWire extends BitWire {

	public VariableBitWire(int wireId) {
		super(wireId);
	}

	public WireArray getBitWires() {
		return new WireArray(new Wire[] { this });
	}

}
