<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_Entre_Calles_30</name>
   <tag></tag>
   <elementGuidId>1c5b002a-d36e-4067-a54f-acc7e906645e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize2>
         &lt;!--Optional:-->
         &lt;addressNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>a1edeae2a5bd4cde241fdfdb193ca13c&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddress>
               &lt;!--Optional:-->
               &lt;level1>&lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>caba&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>AVENIDA COSTANERA DOCTOR TRISTAN ACHAVAL RODRIGUEZ 1508&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber> &lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>&lt;/postalCode>
               &lt;!--Optional:-->
               &lt;additionalData>&lt;/additionalData>
               &lt;!--Optional:-->
               &lt;betweenStreet1>&lt;/betweenStreet1>
               &lt;!--Optional:-->
               &lt;betweenStreet2>&lt;/betweenStreet2>
            &lt;/xAddress>
         &lt;/addressNormalize>
      &lt;/soap:addressNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_ARG2</defaultValue>
      <description></description>
      <id>1398d2bc-b06a-4075-891a-de3127067976</id>
      <masked>false</masked>
      <name>Address_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

/**verificacion test case**/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'CIUDAD AUTONOMA BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'PUERTO MADERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA COSTANERA DR TRISTAN ACHAVAL RODRIGUEZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1508')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.617240')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.356545')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1107')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'C1107ACR')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1101')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '2200')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'DRA ELVIRA RAWSON DE DELLEPIANE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'PADRE MARIO LUIS MIGONE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>6&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.607161&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.445288&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonallevel5abbreviated12&lt;/name>&lt;value>PTO MADERO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet30&lt;/name>&lt;value>AVENIDA ACHAVAL RODRIGUEZ&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>beteewStreet1Abbreviated24&lt;/name>&lt;value>DRA ELVIRA RAWSON DE DELLEPIANE&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4abbreviated15&lt;/name>&lt;value>CAPITAL FEDERAL&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>beteewStreet2Abbreviated24&lt;/name>&lt;value>PADRE MARIO LUIS MIGONE&lt;/value>')

/**verificacion WS unitaria 

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'CIUDAD AUTONOMA BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'PUERTO MADERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA COSTANERA DR TRISTAN ACHAVAL RODRIGUEZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1508')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.617240')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.356545')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1107')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'C1107ACR')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1101')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '2200')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'DRA ELVIRA RAWSON DE DELLEPIANE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'PADRE MARIO LUIS MIGONE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>6&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.607161&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.445288&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>zonallevel5abbreviated12&lt;/name>&lt;value>PTO MADERO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet30&lt;/name>&lt;value>AVENIDA ACHAVAL RODRIGUEZ&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>beteewStreet1Abbreviated24&lt;/name>&lt;value>DRA ELVIRA RAWSON DE DELLEPIANE&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4abbreviated15&lt;/name>&lt;value>CAPITAL FEDERAL&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>beteewStreet2Abbreviated24&lt;/name>&lt;value>PADRE MARIO LUIS MIGONE&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
