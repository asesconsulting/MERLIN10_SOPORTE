<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Patentes_SD_FedePat</name>
   <tag></tag>
   <elementGuidId>14fe4e1d-796f-44fe-b2d4-3eaf06e6dc43</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.vehicleonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:infoDomainService>
         &lt;!--Optional:-->
         &lt;vehicleData>
            &lt;!--1 or more repetitions:-->
            &lt;customValues>
               &lt;!--Optional:-->
               &lt;name> &lt;/name>
               &lt;!--Optional:-->
               &lt;value> &lt;/value>
            &lt;/customValues>
            &lt;!--Optional:-->
            &lt;dominio>AA004FG&lt;/dominio>
            &lt;!--Optional:-->
            &lt;motor> &lt;/motor>
            &lt;!--Optional:-->
            &lt;nroChasis> &lt;/nroChasis>
            &lt;!--Optional:-->
            &lt;clientAccessCode>ea86ad3f0ed110b6961d1b883688534c&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;sector> &lt;/sector>
            &lt;!--Optional:-->
            &lt;userName> &lt;/userName>
         &lt;/vehicleData>
      &lt;/soap:infoDomainService>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>infoDomainService</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Patentes</defaultValue>
      <description></description>
      <id>8d8879ac-9dcc-417c-b31a-034dc29d7bce</id>
      <masked>false</masked>
      <name>Patentes</name>
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



WS.verifyElementText(response, 'infoDomainServiceResponse.return.ano', '2016')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.dominio', 'AA004FG')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.marca', 'TOYOTA')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.modelo', 'ETIOS X 1.5 M/T')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.motor', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nroChasis', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.tipo', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.estado', 'SD')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.motivo', 'SM')</verificationScript>
   <wsdlAddress>${Patentes}</wsdlAddress>
</WebServiceRequestEntity>
