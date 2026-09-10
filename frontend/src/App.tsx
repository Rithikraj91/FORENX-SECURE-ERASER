import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

function App() {
  const [message, setMessage] = useState('Ready to test')

  const testDriveEraser = async () => {
    try {
      setMessage('Running mock drive erasure...')

      const result = await invoke('run_mock_drive_erasure', {
        input: {
          target_device_id: 'MockDrive_Disk1',
          physical_drive_index: 1,
          classification: 'Hdd',
          is_system: 'UnSupported',
          is_boot: 'Unsupported',
          is_removable: 'Unsupported',
          is_read_only: 'Unsupported',
          requires_conservatism: false,
          selected_method: 'AtaSanitizeOverwrite',
          test_seed: 'step7-test',
        },
      })

      setMessage(JSON.stringify(result, null, 2))
    } catch (error) {
      setMessage(`Error: ${String(error)}`)
    }
  }

  return (
    <div>
      <h1>ForenX Secure Eraser</h1>

      <p>Step 7.2 — Drive Eraser Integration Test</p>

      <button type="button" onClick={testDriveEraser}>
        Test Mock Drive Eraser
      </button>

      <pre>{message}</pre>
    </div>
  )
}

export default App